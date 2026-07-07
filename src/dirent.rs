//! `<dirent.h>` — directory traversal.
//!
//! Real `opendir`/`readdir`/`closedir`, implemented over `openat` +
//! `getdents64`. A `DIR` owns the directory fd and a kernel-record buffer that
//! `readdir` refills and parses one `linux_dirent64` at a time.
//!
//! Not yet: thread safety, `readdir_r`, `rewinddir`/`seekdir`, `telldir`.

use crate::fcntl::{AT_FDCWD, O_CLOEXEC, O_DIRECTORY, O_RDONLY};
use crate::platform::linux::{nr, syscall3, syscall4};
use crate::types::{c_char, c_int, c_uchar, ino_t, off_t};

// d_type values.
pub const DT_UNKNOWN: c_uchar = 0;
pub const DT_FIFO: c_uchar = 1;
pub const DT_CHR: c_uchar = 2;
pub const DT_DIR: c_uchar = 4;
pub const DT_BLK: c_uchar = 6;
pub const DT_REG: c_uchar = 8;
pub const DT_LNK: c_uchar = 10;
pub const DT_SOCK: c_uchar = 12;

/// `struct dirent`. `d_name` is a fixed 256-byte field (NAME_MAX + 1).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_off: off_t,
    pub d_reclen: u16,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}

const BUF_SIZE: usize = 4096;

/// Directory stream. Allocated on the heap by `opendir`.
#[repr(C)]
pub struct DIR {
    fd: c_int,
    // Cursor into `buf`: `[pos, end)` holds undelivered kernel records.
    pos: usize,
    end: usize,
    // Scratch `dirent` handed back to the caller by `readdir`.
    entry: dirent,
    buf: [u8; BUF_SIZE],
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn opendir(name: *const c_char) -> *mut DIR {
    let fd = unsafe {
        syscall4(
            nr::OPENAT,
            AT_FDCWD as usize,
            name as usize,
            (O_RDONLY | O_DIRECTORY | O_CLOEXEC) as usize,
            0,
        )
    };
    if fd < 0 {
        return core::ptr::null_mut();
    }
    let dir = unsafe { crate::malloc::malloc(core::mem::size_of::<DIR>()) } as *mut DIR;
    if dir.is_null() {
        unsafe { crate::unistd::close(fd as c_int) };
        return core::ptr::null_mut();
    }
    unsafe {
        (*dir).fd = fd as c_int;
        (*dir).pos = 0;
        (*dir).end = 0;
    }
    dir
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn readdir(dir: *mut DIR) -> *mut dirent {
    if dir.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        // Refill the buffer when the current batch is exhausted.
        if (*dir).pos >= (*dir).end {
            let n = syscall3(
                nr::GETDENTS64,
                (*dir).fd as usize,
                (*dir).buf.as_mut_ptr() as usize,
                BUF_SIZE,
            );
            if n <= 0 {
                // 0 == end of directory; <0 == error (errno already set).
                return core::ptr::null_mut();
            }
            (*dir).end = n as usize;
            (*dir).pos = 0;
        }

        // Parse one linux_dirent64 at the cursor:
        //   u64 d_ino; i64 d_off; u16 d_reclen; u8 d_type; char d_name[];
        let rec = (*dir).buf.as_ptr().add((*dir).pos);
        let d_ino = core::ptr::read_unaligned(rec as *const u64);
        let d_off = core::ptr::read_unaligned(rec.add(8) as *const i64);
        let d_reclen = core::ptr::read_unaligned(rec.add(16) as *const u16);
        let d_type = *rec.add(18);
        let name = rec.add(19) as *const c_char;

        (*dir).entry.d_ino = d_ino;
        (*dir).entry.d_off = d_off;
        (*dir).entry.d_reclen = d_reclen;
        (*dir).entry.d_type = d_type;
        // Copy the NUL-terminated name, bounded by the field size.
        let mut i = 0;
        while i < 255 {
            let ch = *name.add(i);
            (*dir).entry.d_name[i] = ch;
            if ch == 0 {
                break;
            }
            i += 1;
        }
        (*dir).entry.d_name[255] = 0;

        (*dir).pos += d_reclen as usize;
        &mut (*dir).entry
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn closedir(dir: *mut DIR) -> c_int {
    if dir.is_null() {
        crate::errno::set_errno(crate::errno::EBADF);
        return -1;
    }
    unsafe {
        let fd = (*dir).fd;
        crate::malloc::free(dir as *mut core::ffi::c_void);
        crate::unistd::close(fd)
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn dirfd(dir: *mut DIR) -> c_int {
    if dir.is_null() {
        crate::errno::set_errno(crate::errno::EINVAL);
        return -1;
    }
    unsafe { (*dir).fd }
}
