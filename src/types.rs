//! Canonical C / POSIX type aliases used throughout the crate.
//!
//! The primitive C types come from [`core::ffi`] so that widths always match
//! the target's C ABI. POSIX types (`off_t`, `pid_t`, ...) are defined here with
//! their Linux LP64 widths, which are identical on the two arches we currently
//! target (x86_64 and aarch64).

pub use core::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort, c_void,
};

// --- Sizes and offsets -----------------------------------------------------

/// Unsigned integer type of the result of `sizeof` (C `size_t`).
pub type size_t = usize;
/// Signed counterpart of `size_t` (C `ssize_t`).
pub type ssize_t = isize;
/// Signed integer type able to hold a pointer difference (C `ptrdiff_t`).
pub type ptrdiff_t = isize;
/// Signed integer type able to hold any pointer (C `intptr_t`).
pub type intptr_t = isize;
/// Unsigned integer type able to hold any pointer (C `uintptr_t`).
pub type uintptr_t = usize;

// --- POSIX numeric types (Linux, LP64) -------------------------------------

pub type off_t = c_long;
pub type off64_t = i64;
pub type mode_t = c_uint;
pub type pid_t = c_int;
pub type uid_t = c_uint;
pub type gid_t = c_uint;
pub type id_t = c_uint;
pub type dev_t = u64;
pub type ino_t = u64;
pub type ino64_t = u64;
pub type nlink_t = u64;
pub type blksize_t = c_long;
pub type blkcnt_t = c_long;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type socklen_t = c_uint;
pub type sa_family_t = c_ushort;

// --- Time ------------------------------------------------------------------

pub type time_t = c_long;
pub type clock_t = c_long;
pub type clockid_t = c_int;
pub type suseconds_t = c_long;
pub type useconds_t = c_uint;

/// C `struct timespec` — seconds + nanoseconds. Matches the kernel layout.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

/// C `struct timeval` — seconds + microseconds.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

// --- Wide characters -------------------------------------------------------

/// Wide character. On Linux `wchar_t` is a signed 32-bit int.
pub type wchar_t = i32;
/// Wide int / EOF-carrying wide char (C `wint_t`).
pub type wint_t = u32;

// --- Misc ------------------------------------------------------------------

/// A `NULL`-safe void pointer alias for readability in signatures.
pub type c_void_ptr = *mut c_void;
