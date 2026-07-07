//! `<stdlib.h>` — numeric conversion, process control, search/sort, environment.
//!
//! Allocation (`malloc` and friends) lives in [`crate::malloc`]. This module
//! re-exports nothing from there; the header aggregates them.

use crate::types::{c_char, c_int, c_long, c_longlong, c_ulong, c_void, size_t};

/// `EXIT_SUCCESS` / `EXIT_FAILURE`.
pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;
/// Upper bound of `rand`.
pub const RAND_MAX: c_int = 0x7fff_ffff;

// --- string -> integer -----------------------------------------------------

#[inline]
unsafe fn skip_ws(mut p: *const c_char) -> *const c_char {
    while crate::ctype::isspace(unsafe { *p } as c_int) != 0 {
        p = unsafe { p.add(1) };
    }
    p
}

/// Core signed parser shared by `strtol`/`strtoll`/`atoi`. Honors an optional
/// sign, `base` in 0/2..=36 (0 = auto-detect `0x`/`0` prefixes), and stops at
/// the first non-digit, writing the stop position to `endptr` when non-NULL.
unsafe fn strto_signed(nptr: *const c_char, endptr: *mut *mut c_char, mut base: c_int) -> i64 {
    let mut p = unsafe { skip_ws(nptr) };
    let mut neg = false;
    match unsafe { *p } as u8 {
        b'+' => p = unsafe { p.add(1) },
        b'-' => {
            neg = true;
            p = unsafe { p.add(1) };
        }
        _ => {}
    }
    // Prefix handling for base 0 / 16.
    if (base == 0 || base == 16)
        && unsafe { *p } as u8 == b'0'
        && matches!(unsafe { *p.add(1) } as u8, b'x' | b'X')
    {
        p = unsafe { p.add(2) };
        base = 16;
    } else if base == 0 && unsafe { *p } as u8 == b'0' {
        base = 8;
    } else if base == 0 {
        base = 10;
    }

    let mut acc: i64 = 0;
    loop {
        let ch = unsafe { *p } as u8;
        let digit = match ch {
            b'0'..=b'9' => (ch - b'0') as c_int,
            b'a'..=b'z' => (ch - b'a' + 10) as c_int,
            b'A'..=b'Z' => (ch - b'A' + 10) as c_int,
            _ => break,
        };
        if digit >= base {
            break;
        }
        acc = acc * base as i64 + digit as i64;
        p = unsafe { p.add(1) };
    }
    if !endptr.is_null() {
        unsafe { *endptr = p as *mut c_char };
    }
    if neg { -acc } else { acc }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long {
    unsafe { strto_signed(nptr, endptr, base) as c_long }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strtoll(
    nptr: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> c_longlong {
    unsafe { strto_signed(nptr, endptr, base) as c_longlong }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strtoul(
    nptr: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> c_ulong {
    unsafe { strto_signed(nptr, endptr, base) as c_ulong }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn atoi(nptr: *const c_char) -> c_int {
    unsafe { strto_signed(nptr, core::ptr::null_mut(), 10) as c_int }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn atol(nptr: *const c_char) -> c_long {
    unsafe { strto_signed(nptr, core::ptr::null_mut(), 10) as c_long }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn atoll(nptr: *const c_char) -> c_longlong {
    unsafe { strto_signed(nptr, core::ptr::null_mut(), 10) as c_longlong }
}

// --- integer arithmetic ----------------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn abs(j: c_int) -> c_int {
    j.wrapping_abs()
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn labs(j: c_long) -> c_long {
    j.wrapping_abs()
}

// --- search & sort ---------------------------------------------------------

/// Comparator type shared by `qsort`/`bsearch`.
pub type CompareFn = unsafe extern "C" fn(*const c_void, *const c_void) -> c_int;

/// `bsearch` — binary search over a sorted array.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn bsearch(
    key: *const c_void,
    base: *const c_void,
    nmemb: size_t,
    size: size_t,
    compar: CompareFn,
) -> *mut c_void {
    let mut lo = 0isize;
    let mut hi = nmemb as isize - 1;
    let base = base as *const u8;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let elem = unsafe { base.offset(mid * size as isize) };
        let ord = unsafe { compar(key, elem as *const c_void) };
        if ord < 0 {
            hi = mid - 1;
        } else if ord > 0 {
            lo = mid + 1;
        } else {
            return elem as *mut c_void;
        }
    }
    core::ptr::null_mut()
}

/// `qsort` — sort an array in place. Uses an insertion sort for now: simple and
/// correct for the scaffold; a later pass swaps in an introsort/quicksort.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn qsort(
    base: *mut c_void,
    nmemb: size_t,
    size: size_t,
    compar: CompareFn,
) {
    if nmemb < 2 || size == 0 {
        return;
    }
    let base = base as *mut u8;
    let elem = |i: usize| unsafe { base.add(i * size) };
    for i in 1..nmemb {
        let mut j = i;
        while j > 0 {
            let a = elem(j - 1);
            let b = elem(j);
            if unsafe { compar(a as *const c_void, b as *const c_void) } <= 0 {
                break;
            }
            // Swap the two `size`-byte elements byte by byte.
            for k in 0..size {
                unsafe {
                    let t = *a.add(k);
                    *a.add(k) = *b.add(k);
                    *b.add(k) = t;
                }
            }
            j -= 1;
        }
    }
}

// --- process control -------------------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn exit(status: c_int) -> ! {
    // TODO: run atexit handlers and flush stdio before terminating.
    unsafe { crate::unistd::_exit(status) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn abort() -> ! {
    // Raise SIGABRT; until signal delivery is wired up, exit with 134 (128+6),
    // the conventional shell status for an aborted process.
    unsafe { crate::unistd::_exit(134) }
}

/// `__assert_fail` — target of the `assert()` macro when the condition is false.
/// Prints "file:line: func: Assertion `expr` failed." to stderr and aborts.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn __assert_fail(
    expr: *const c_char,
    file: *const c_char,
    line: c_int,
    func: *const c_char,
) -> ! {
    use crate::stdio::{fputs, stderr};
    unsafe {
        if !file.is_null() {
            fputs(file, stderr);
            fputs(b":\0".as_ptr() as *const c_char, stderr);
        }
        // Render the line number (small helper; avoids depending on printf).
        let mut buf = [0u8; 20];
        let mut i = buf.len();
        let mut n = line as u32;
        if n == 0 {
            i -= 1;
            buf[i] = b'0';
        }
        while n > 0 {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }
        crate::unistd::write(
            2,
            buf.as_ptr().add(i) as *const c_void,
            buf.len() - i,
        );
        fputs(b": \0".as_ptr() as *const c_char, stderr);
        if !func.is_null() {
            fputs(func, stderr);
            fputs(b": \0".as_ptr() as *const c_char, stderr);
        }
        fputs(b"Assertion `\0".as_ptr() as *const c_char, stderr);
        if !expr.is_null() {
            fputs(expr, stderr);
        }
        fputs(b"' failed.\n\0".as_ptr() as *const c_char, stderr);
    }
    abort()
}

/// `getenv` — STUB. The environment vector isn't captured yet (needs crt to
/// stash `envp`). Always reports "not found".
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn getenv(_name: *const c_char) -> *mut c_char {
    // STUB
    core::ptr::null_mut()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ints() {
        unsafe {
            assert_eq!(atoi(b"  -42xyz\0".as_ptr() as _), -42);
            assert_eq!(strtol(b"0x1F\0".as_ptr() as _, core::ptr::null_mut(), 0), 31);
            assert_eq!(strtol(b"777\0".as_ptr() as _, core::ptr::null_mut(), 8), 511);
        }
    }

    #[test]
    fn sort_then_search() {
        unsafe extern "C" fn cmp(a: *const c_void, b: *const c_void) -> c_int {
            let x = unsafe { *(a as *const c_int) };
            let y = unsafe { *(b as *const c_int) };
            x - y
        }
        let mut arr: [c_int; 5] = [5, 3, 1, 4, 2];
        unsafe {
            qsort(
                arr.as_mut_ptr() as *mut c_void,
                5,
                core::mem::size_of::<c_int>(),
                cmp,
            );
        }
        assert_eq!(arr, [1, 2, 3, 4, 5]);
        let key: c_int = 4;
        let found = unsafe {
            bsearch(
                &key as *const c_int as *const c_void,
                arr.as_ptr() as *const c_void,
                5,
                core::mem::size_of::<c_int>(),
                cmp,
            )
        };
        assert!(!found.is_null());
        assert_eq!(unsafe { *(found as *const c_int) }, 4);
    }
}
