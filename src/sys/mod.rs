//! `sys/*` headers.
//!
//! Groups the POSIX `sys/…` interfaces under one Rust module tree. Each
//! submodule mirrors one header (`sys/mman.h` -> [`mman`], etc.).

pub mod mman;
pub mod stat;
pub mod uio;
pub mod utsname;
pub mod wait;
