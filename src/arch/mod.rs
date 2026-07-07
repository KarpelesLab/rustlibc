//! Per-architecture primitives.
//!
//! Each supported architecture provides:
//! - `syscall0` … `syscall6`: the raw kernel trap, arguments already in
//!   register-sized form. These are the *only* place inline assembly touches the
//!   syscall instruction; everything above builds on them.
//! - register/entry conventions consumed by [`crate::crt`] and [`crate::setjmp`].
//!
//! The public surface (`syscallN`) is identical across arches so that
//! [`crate::platform`] can be written once against it.

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
compile_error!("rustlibc currently supports only x86_64 and aarch64");
