//! `<locale.h>` — localization.
//!
//! Only the "C"/"POSIX" locale is supported. `setlocale` is a **stub** that
//! always reports the C locale; `localeconv` returns a static C-locale
//! description. Marked `STUB` where behavior is nominal.

use crate::types::{c_char, c_int};

pub const LC_ALL: c_int = 6;
pub const LC_COLLATE: c_int = 3;
pub const LC_CTYPE: c_int = 0;
pub const LC_MONETARY: c_int = 4;
pub const LC_NUMERIC: c_int = 1;
pub const LC_TIME: c_int = 2;

/// `setlocale` — STUB. We only implement the C locale; always report it.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn setlocale(_category: c_int, _locale: *const c_char) -> *mut c_char {
    // STUB: the returned string must not be modified by the caller.
    b"C\0".as_ptr() as *mut c_char
}
