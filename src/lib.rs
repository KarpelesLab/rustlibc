//! rustlibc — a full libc implementation as a single Rust crate.
//!
//! The crate is freestanding (`#![no_std]`) and talks to the kernel directly
//! through raw syscalls (see [`arch`]/[`platform`]). Every user-visible libc
//! function is exported with the C ABI under its canonical name, so the
//! resulting `librustlibc.a` / `librustlibc.so` can be linked by ordinary
//! C/C++ programs in place of glibc or musl.
//!
//! ## Layout
//! - [`arch`]     — per-architecture primitives (raw syscall, `setjmp`, `_start`).
//! - [`platform`] — per-OS glue (syscall numbers, kernel struct layouts).
//! - the remaining modules mirror the C standard / POSIX headers one-to-one
//!   (`string`, `stdio`, `stdlib`, ...). Each `pub` item that carries a C name
//!   is annotated `#[unsafe(no_mangle)] extern "C"`.
//!
//! Status: this is an early scaffold. Pure-computation surfaces (string, memory,
//! ctype) are implemented; I/O, allocation and math are partially real and
//! partially stubbed. Stubs are marked `// STUB` and set `errno = ENOSYS`.

// Freestanding in every real build. Under `cfg(test)` we link the std test
// harness instead (it needs an unwinder and its own panic handler), so the
// crate is only `no_std` when not testing. Our C-ABI exports and the panic
// handler are likewise gated `not(test)` so nothing collides with the harness.
#![cfg_attr(not(test), no_std)]
// Consume C `va_list`s (printf/scanf family). Nightly-only; see rust-toolchain.toml.
#![feature(c_variadic)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]
// A libc *is* the collection of dangerous primitives; per-fn safety docs would
// be noise. Keep the lint off crate-wide and document invariants where subtle.
#![allow(clippy::manual_c_str_literals)]
// Internal message/format literals are written as explicit NUL-terminated byte
// strings (`b"...\0"`) — the canonical form for passing `*const c_char` to our
// own routines. This is intentional and pervasive; the lint would only add noise.

// Architecture and OS layers.
pub mod arch;
pub mod platform;

// Core type + error surface shared by every module.
pub mod errno;
pub mod types;

// C standard / POSIX headers, one module per header.
pub mod ctype;
pub mod fcntl;
pub mod malloc;
pub mod math;
pub mod setjmp;
pub mod signal;
pub mod stdio;
pub mod stdlib;
pub mod string;
pub mod strings;
pub mod time;
pub mod unistd;
pub mod wchar;

// C runtime bootstrap (`_start`, `__libc_start_main`). Owns the program entry
// point, so it must be disabled when linking against another libc — and under
// `cfg(test)`, where the std harness already provides `_start`/crt1.
#[cfg(all(feature = "crt", not(test)))]
pub mod crt;

/// Freestanding panic handler.
///
/// Present only in real (non-`cargo test`) builds; under `cfg(test)` the crate
/// is linked with the std test harness, which supplies its own handler.
///
/// A libc has nowhere to unwind *to*, so we abort the process. All profiles set
/// `panic = "abort"`, meaning this is only reached via an explicit `panic!`
/// (e.g. a failed invariant), never from `extern "C"` entry points.
#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // Cannot format safely here without risking re-entrancy; just die loudly.
    unsafe { crate::unistd::_exit(127) }
}

/// Unwinding personality routine.
///
/// All profiles use `panic = "abort"`, so this is never called. But the
/// precompiled `core` shipped by rustup is built with `panic = "unwind"` and
/// carries a reference to `rust_eh_personality`; providing this empty symbol
/// satisfies the linker for freestanding staticlib/cdylib links.
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}
