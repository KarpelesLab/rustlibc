//! aarch64 (AArch64 / Linux) architecture primitives.
//!
//! Linux syscall convention on aarch64:
//! - syscall number in `x8`
//! - arguments in `x0, x1, x2, x3, x4, x5`
//! - return value in `x0`
//! - `svc #0` performs the trap; it clobbers no general-purpose registers
//!   beyond the return in `x0`
//!
//! As on x86_64, errors come back as `x0` in `-4095 ..= -1`.

use core::arch::asm;

#[inline]
pub unsafe fn syscall0(n: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "svc #0",
            in("x8") n,
            lateout("x0") ret,
            options(nostack, preserves_flags),
        );
    }
    ret
}

#[inline]
pub unsafe fn syscall1(n: usize, a1: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "svc #0",
            in("x8") n,
            inlateout("x0") a1 => ret,
            options(nostack, preserves_flags),
        );
    }
    ret
}

#[inline]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "svc #0",
            in("x8") n,
            inlateout("x0") a1 => ret,
            in("x1") a2,
            options(nostack, preserves_flags),
        );
    }
    ret
}

#[inline]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "svc #0",
            in("x8") n,
            inlateout("x0") a1 => ret,
            in("x1") a2,
            in("x2") a3,
            options(nostack, preserves_flags),
        );
    }
    ret
}

#[inline]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "svc #0",
            in("x8") n,
            inlateout("x0") a1 => ret,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            options(nostack, preserves_flags),
        );
    }
    ret
}

#[inline]
pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "svc #0",
            in("x8") n,
            inlateout("x0") a1 => ret,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            in("x4") a5,
            options(nostack, preserves_flags),
        );
    }
    ret
}

#[inline]
pub unsafe fn syscall6(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "svc #0",
            in("x8") n,
            inlateout("x0") a1 => ret,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            in("x4") a5,
            in("x5") a6,
            options(nostack, preserves_flags),
        );
    }
    ret
}
