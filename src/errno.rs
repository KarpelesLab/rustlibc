//! `errno` and the E* error constants.
//!
//! C programs access `errno` as an lvalue; glibc/musl implement that macro as
//! `*__errno_location()`. We do the same: [`__errno_location`] returns a pointer
//! to the current thread's `errno` slot, and the `errno` macro in `<errno.h>`
//! expands to `(*__errno_location())`.
//!
//! TODO(threads): the slot is currently a single process-global. Once TLS is
//! wired up (`arch` set_thread_pointer + a TCB), move it into the TCB so each
//! thread sees its own `errno`.

use crate::types::c_int;

/// Process-global `errno` storage. Not yet per-thread — see module docs.
static mut ERRNO: c_int = 0;

/// Return a pointer to the calling thread's `errno`.
///
/// This is the ABI contract every C program relies on for `errno`.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn __errno_location() -> *mut c_int {
    &raw mut ERRNO
}

/// Set `errno` from within the crate.
#[inline]
pub fn set_errno(value: c_int) {
    unsafe { *__errno_location() = value }
}

/// Read `errno` from within the crate.
#[inline]
pub fn errno() -> c_int {
    unsafe { *__errno_location() }
}

// --- E* constants (Linux generic ABI; identical on x86_64 and aarch64) ------

pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const ENOTBLK: c_int = 15;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOTTY: c_int = 25;
pub const ETXTBSY: c_int = 26;
pub const EFBIG: c_int = 27;
pub const ENOSPC: c_int = 28;
pub const ESPIPE: c_int = 29;
pub const EROFS: c_int = 30;
pub const EMLINK: c_int = 31;
pub const EPIPE: c_int = 32;
pub const EDOM: c_int = 33;
pub const ERANGE: c_int = 34;
pub const EDEADLK: c_int = 35;
pub const ENAMETOOLONG: c_int = 36;
pub const ENOLCK: c_int = 37;
pub const ENOSYS: c_int = 38;
pub const ENOTEMPTY: c_int = 39;
pub const ELOOP: c_int = 40;
pub const ENOMSG: c_int = 42;
pub const EOVERFLOW: c_int = 75;
pub const EOPNOTSUPP: c_int = 95;
pub const ETIMEDOUT: c_int = 110;

/// `EWOULDBLOCK` is an alias for `EAGAIN` on Linux.
pub const EWOULDBLOCK: c_int = EAGAIN;
