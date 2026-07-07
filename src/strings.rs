//! `<strings.h>` — the older BSD/POSIX case-insensitive and bit helpers.
//!
//! Distinct from `<string.h>`. Fully implemented — pure computation.

use crate::ctype::tolower;
use crate::types::{c_char, c_int, c_void, size_t};

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int {
    let mut i = 0;
    loop {
        let x = tolower(unsafe { *a.add(i) } as u8 as c_int);
        let y = tolower(unsafe { *b.add(i) } as u8 as c_int);
        if x != y || x == 0 {
            return x - y;
        }
        i += 1;
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strncasecmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int {
    let mut i = 0;
    while i < n {
        let x = tolower(unsafe { *a.add(i) } as u8 as c_int);
        let y = tolower(unsafe { *b.add(i) } as u8 as c_int);
        if x != y || x == 0 {
            return x - y;
        }
        i += 1;
    }
    0
}

/// `bzero` (legacy) — zero `n` bytes. Prefer `memset`.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn bzero(s: *mut c_void, n: size_t) {
    unsafe { crate::string::memset(s, 0, n) };
}

/// `bcopy` (legacy) — note the reversed argument order vs `memcpy`, and that it
/// is defined for overlapping regions (so it behaves like `memmove`).
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn bcopy(src: *const c_void, dest: *mut c_void, n: size_t) {
    unsafe { crate::string::memmove(dest, src, n) };
}

/// `ffs` — index (1-based) of the least-significant set bit, 0 if none.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn ffs(i: c_int) -> c_int {
    if i == 0 {
        0
    } else {
        i.trailing_zeros() as c_int + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casecmp() {
        unsafe {
            assert_eq!(
                strcasecmp(b"Hello\0".as_ptr() as _, b"hELLO\0".as_ptr() as _),
                0
            );
        }
    }

    #[test]
    fn ffs_basic() {
        assert_eq!(ffs(0), 0);
        assert_eq!(ffs(1), 1);
        assert_eq!(ffs(0b1000), 4);
    }
}
