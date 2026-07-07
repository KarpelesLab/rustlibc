//! `<sys/mman.h>` — memory management.
//!
//! Real wrappers over the `mmap`/`munmap`/`mprotect`/`madvise` syscalls. The
//! allocator ([`crate::malloc`]) uses `mmap` directly; this exposes the POSIX
//! surface to C callers.

use crate::platform::linux::{nr, syscall2, syscall3, syscall6};
use crate::types::{c_int, c_void, off_t, size_t};

pub const PROT_NONE: c_int = 0x0;
pub const PROT_READ: c_int = 0x1;
pub const PROT_WRITE: c_int = 0x2;
pub const PROT_EXEC: c_int = 0x4;

pub const MAP_SHARED: c_int = 0x01;
pub const MAP_PRIVATE: c_int = 0x02;
pub const MAP_FIXED: c_int = 0x10;
pub const MAP_ANONYMOUS: c_int = 0x20;

/// `mmap` failure sentinel: `(void *) -1`.
pub const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn mmap(
    addr: *mut c_void,
    length: size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: off_t,
) -> *mut c_void {
    let ret = unsafe {
        syscall6(
            nr::MMAP,
            addr as usize,
            length,
            prot as usize,
            flags as usize,
            fd as usize,
            offset as usize,
        )
    };
    if ret < 0 {
        // syscall_ret already set errno; report the POSIX sentinel.
        MAP_FAILED
    } else {
        ret as usize as *mut c_void
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn munmap(addr: *mut c_void, length: size_t) -> c_int {
    unsafe { syscall2(nr::MUNMAP, addr as usize, length) as c_int }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn mprotect(addr: *mut c_void, length: size_t, prot: c_int) -> c_int {
    unsafe { syscall3(nr::MPROTECT, addr as usize, length, prot as usize) as c_int }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int {
    unsafe { syscall3(nr::MADVISE, addr as usize, length, advice as usize) as c_int }
}
