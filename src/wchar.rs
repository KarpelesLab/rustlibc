//! `<wchar.h>` — wide-character strings.
//!
//! The length/compare/copy primitives are implemented (pure computation over
//! `wchar_t`). Multibyte<->wide conversion (`mbrtowc`, `wcrtomb`) and wide I/O
//! are **stubbed** pending a locale/UTF-8 codec. Marked `STUB`.

use crate::types::{size_t, wchar_t};

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn wcslen(s: *const wchar_t) -> size_t {
    let mut n = 0;
    while unsafe { *s.add(n) } != 0 {
        n += 1;
    }
    n
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn wcscmp(a: *const wchar_t, b: *const wchar_t) -> crate::types::c_int {
    let mut i = 0;
    loop {
        let (x, y) = unsafe { (*a.add(i), *b.add(i)) };
        if x != y || x == 0 {
            return (x - y) as crate::types::c_int;
        }
        i += 1;
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn wcscpy(dest: *mut wchar_t, src: *const wchar_t) -> *mut wchar_t {
    let mut i = 0;
    loop {
        let ch = unsafe { *src.add(i) };
        unsafe { *dest.add(i) = ch };
        if ch == 0 {
            break;
        }
        i += 1;
    }
    dest
}

/// `mbrtowc` — STUB. Needs a UTF-8 (or locale) decoder.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn mbrtowc(
    _pwc: *mut wchar_t,
    _s: *const crate::types::c_char,
    _n: size_t,
    _ps: *mut core::ffi::c_void,
) -> size_t {
    // STUB
    crate::errno::set_errno(crate::errno::ENOSYS);
    usize::MAX // (size_t)-1: encoding error, per the C spec
}
