//! `<sys/uio.h>` — scatter/gather I/O.
//!
//! Real wrappers over `readv`/`writev`.

use crate::platform::linux::{nr, syscall3};
use crate::types::{c_int, c_void, size_t, ssize_t};

/// A single scatter/gather buffer (`struct iovec`).
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn readv(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t {
    unsafe { syscall3(nr::READV, fd as usize, iov as usize, iovcnt as usize) as ssize_t }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t {
    unsafe { syscall3(nr::WRITEV, fd as usize, iov as usize, iovcnt as usize) as ssize_t }
}
