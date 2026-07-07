//! C runtime startup (`crt0`).
//!
//! `_start` is the ELF entry point. The kernel hands control here with the
//! initial process stack laid out as: `[argc][argv[0]]…[NULL][envp[0]]…[NULL]`
//! and `sp` pointing at `argc`. The naked `_start` extracts `argc`/`argv`/`envp`
//! into the C calling convention and tail-calls [`__libc_start_main`], which
//! runs `main` and then `exit`s with its result.
//!
//! This module is gated behind the `crt` feature: disable it when linking
//! rustlibc alongside another libc that already owns the process entry point.

use crate::types::{c_char, c_int};

// The application's entry point. Defined by the program being linked; the
// reference is resolved at final link time.
unsafe extern "C" {
    fn main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;
}

// Captured program vectors, stashed for `getenv`/`__environ` once those are
// wired up. Written once at startup, before any user code runs.
pub(crate) static mut ARGC: c_int = 0;
pub(crate) static mut ARGV: *mut *mut c_char = core::ptr::null_mut();
pub(crate) static mut ENVIRON: *mut *mut c_char = core::ptr::null_mut();

/// The `environ` global that POSIX programs may reference directly.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub static mut environ: *mut *mut c_char = core::ptr::null_mut();

/// Bootstrap: record the process vectors, run `main`, and terminate with its
/// return value. Never returns.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn __libc_start_main(
    argc: c_int,
    argv: *mut *mut c_char,
    envp: *mut *mut c_char,
) -> ! {
    unsafe {
        ARGC = argc;
        ARGV = argv;
        ENVIRON = envp;
        environ = envp;

        // TODO: run .init_array constructors and register stdio flush at exit.
        let code = main(argc, argv, envp);
        crate::stdlib::exit(code)
    }
}

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "xor ebp, ebp",              // outermost frame: clear rbp
        "mov rdi, [rsp]",            // argc
        "lea rsi, [rsp + 8]",        // argv
        "lea rdx, [rsi + rdi*8 + 8]", // envp = argv + argc*8 + 8
        "and rsp, -16",              // 16-byte align before the call
        "call {start_main}",
        "hlt",                        // __libc_start_main never returns
        start_main = sym __libc_start_main,
    )
}

#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "mov x29, #0",          // clear frame pointer
        "mov x30, #0",          // clear link register
        "ldr x0, [sp]",         // argc
        "add x1, sp, #8",       // argv
        "add x2, x0, #1",       // argc + 1
        "add x2, x1, x2, lsl #3", // envp = argv + (argc+1)*8
        "bl {start_main}",
        "brk #0",                // unreachable
        start_main = sym __libc_start_main,
    )
}
