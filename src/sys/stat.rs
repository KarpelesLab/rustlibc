//! `<sys/stat.h>` — file status.
//!
//! `struct stat` has a **different layout on each architecture** (x86_64 uses
//! the x86-specific kernel `struct stat`; aarch64 uses the asm-generic one), so
//! it is defined per-arch below with a compile-time size check. All of
//! `stat`/`lstat`/`fstat` route through the modern `newfstatat` syscall, which
//! both arches provide, avoiding the legacy per-arch `stat`/`lstat` entry
//! points (aarch64 has none).

use crate::fcntl::AT_FDCWD;
use crate::platform::linux::{nr, syscall3, syscall4};
use crate::types::{
    blkcnt_t, c_char, c_int, c_long, dev_t, gid_t, ino_t, mode_t, off_t, time_t, uid_t,
};

// File-type and permission bits (shared across arches).
pub const S_IFMT: mode_t = 0o170000;
pub const S_IFSOCK: mode_t = 0o140000;
pub const S_IFLNK: mode_t = 0o120000;
pub const S_IFREG: mode_t = 0o100000;
pub const S_IFBLK: mode_t = 0o060000;
pub const S_IFDIR: mode_t = 0o040000;
pub const S_IFCHR: mode_t = 0o020000;
pub const S_IFIFO: mode_t = 0o010000;
pub const S_ISUID: mode_t = 0o4000;
pub const S_ISGID: mode_t = 0o2000;
pub const S_ISVTX: mode_t = 0o1000;
pub const S_IRWXU: mode_t = 0o700;
pub const S_IRWXG: mode_t = 0o070;
pub const S_IRWXO: mode_t = 0o007;

// `newfstatat` flags used by the wrappers.
const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
const AT_EMPTY_PATH: c_int = 0x1000;

/// `struct stat` — x86_64 kernel layout (sizeof == 144).
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_nlink: crate::types::nlink_t,
    pub st_mode: mode_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    __pad0: c_int,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: crate::types::blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_atime: time_t,
    pub st_atime_nsec: c_long,
    pub st_mtime: time_t,
    pub st_mtime_nsec: c_long,
    pub st_ctime: time_t,
    pub st_ctime_nsec: c_long,
    __unused: [c_long; 3],
}

/// `struct stat` — aarch64 (asm-generic) layout (sizeof == 128).
#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_mode: mode_t,
    // aarch64's st_nlink is a 32-bit `unsigned int`, unlike x86_64's `unsigned long`.
    pub st_nlink: crate::types::c_uint,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    __pad1: dev_t,
    pub st_size: off_t,
    pub st_blksize: c_int,
    __pad2: c_int,
    pub st_blocks: blkcnt_t,
    pub st_atime: time_t,
    pub st_atime_nsec: c_long,
    pub st_mtime: time_t,
    pub st_mtime_nsec: c_long,
    pub st_ctime: time_t,
    pub st_ctime_nsec: c_long,
    __unused: [c_int; 2],
}

// Guard against accidental layout drift.
#[cfg(target_arch = "x86_64")]
const _: () = assert!(core::mem::size_of::<stat>() == 144);
#[cfg(target_arch = "aarch64")]
const _: () = assert!(core::mem::size_of::<stat>() == 128);

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fstatat(
    dirfd: c_int,
    path: *const c_char,
    buf: *mut stat,
    flags: c_int,
) -> c_int {
    unsafe {
        syscall4(
            nr::NEWFSTATAT,
            dirfd as usize,
            path as usize,
            buf as usize,
            flags as usize,
        ) as c_int
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn stat(path: *const c_char, buf: *mut stat) -> c_int {
    unsafe { fstatat(AT_FDCWD, path, buf, 0) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn lstat(path: *const c_char, buf: *mut stat) -> c_int {
    unsafe { fstatat(AT_FDCWD, path, buf, AT_SYMLINK_NOFOLLOW) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fstat(fd: c_int, buf: *mut stat) -> c_int {
    // Empty path + AT_EMPTY_PATH makes newfstatat operate on `fd` itself.
    unsafe { fstatat(fd, b"\0".as_ptr() as *const c_char, buf, AT_EMPTY_PATH) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn mkdir(path: *const c_char, mode: mode_t) -> c_int {
    unsafe { syscall3(nr::MKDIRAT, AT_FDCWD as usize, path as usize, mode as usize) as c_int }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn chmod(path: *const c_char, mode: mode_t) -> c_int {
    unsafe {
        syscall4(nr::FCHMODAT, AT_FDCWD as usize, path as usize, mode as usize, 0) as c_int
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fchmod(fd: c_int, mode: mode_t) -> c_int {
    unsafe { crate::platform::linux::syscall2(nr::FCHMOD, fd as usize, mode as usize) as c_int }
}
