//! Linux platform layer.
//!
//! [`syscall`] and [`syscall_ret`] are the two entry points the rest of the
//! crate uses. They accept the syscall number from [`nr`] and up to six
//! `usize`-sized arguments, and (for `syscall_ret`) apply the standard Linux
//! error convention: a return value in `-4095 ..= -1` is `-errno`.

pub mod nr;

use crate::arch;
use crate::errno::set_errno;
use crate::types::{c_int, c_long};

/// Largest magnitude of an in-band error return (`MAX_ERRNO` in the kernel).
const MAX_ERRNO: usize = 4095;

/// Perform a syscall, returning the kernel's raw `usize` result (which may be a
/// negative `-errno` reinterpreted as a large `usize`). Prefer [`syscall_ret`]
/// unless you specifically need the raw value.
///
/// `args` holds up to six arguments; extra slots are ignored.
#[inline]
pub unsafe fn syscall(n: usize, args: [usize; 6]) -> usize {
    unsafe { arch::syscall6(n, args[0], args[1], args[2], args[3], args[4], args[5]) }
}

/// Perform a syscall and translate the result into the libc convention:
/// on error set `errno` and return `-1`, otherwise return the value as `c_long`.
#[inline]
pub unsafe fn syscall_ret(n: usize, args: [usize; 6]) -> c_long {
    let raw = unsafe { syscall(n, args) };
    if raw > usize::MAX - MAX_ERRNO {
        // raw is -1 ..= -4095
        set_errno(-(raw as isize) as c_int);
        -1
    } else {
        raw as c_long
    }
}

// Convenience wrappers so call sites read naturally (`sys::write(...)` style)
// without every caller building a 6-element array by hand.

#[inline]
pub unsafe fn syscall0(n: usize) -> c_long {
    unsafe { syscall_ret(n, [0; 6]) }
}
#[inline]
pub unsafe fn syscall1(n: usize, a: usize) -> c_long {
    unsafe { syscall_ret(n, [a, 0, 0, 0, 0, 0]) }
}
#[inline]
pub unsafe fn syscall2(n: usize, a: usize, b: usize) -> c_long {
    unsafe { syscall_ret(n, [a, b, 0, 0, 0, 0]) }
}
#[inline]
pub unsafe fn syscall3(n: usize, a: usize, b: usize, c: usize) -> c_long {
    unsafe { syscall_ret(n, [a, b, c, 0, 0, 0]) }
}
#[inline]
pub unsafe fn syscall4(n: usize, a: usize, b: usize, c: usize, d: usize) -> c_long {
    unsafe { syscall_ret(n, [a, b, c, d, 0, 0]) }
}
#[inline]
pub unsafe fn syscall5(n: usize, a: usize, b: usize, c: usize, d: usize, e: usize) -> c_long {
    unsafe { syscall_ret(n, [a, b, c, d, e, 0]) }
}
#[inline]
pub unsafe fn syscall6(
    n: usize,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    f: usize,
) -> c_long {
    unsafe { syscall_ret(n, [a, b, c, d, e, f]) }
}
