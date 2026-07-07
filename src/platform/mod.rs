//! Per-OS glue.
//!
//! Only Linux is implemented today. This layer maps the arch-neutral
//! `syscallN` primitives to named operations, owns the kernel syscall-number
//! tables, and turns the kernel's in-band error convention into `errno`.

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(not(target_os = "linux"))]
compile_error!("rustlibc currently supports only target_os = \"linux\"");
