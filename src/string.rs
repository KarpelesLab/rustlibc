//! `<string.h>` — memory and C-string operations.
//!
//! These are pure computation (no syscalls) and are fully implemented. They
//! double as the symbols the Rust compiler itself lowers `memcpy`/`memset`/…
//! calls to, so correctness here underpins the whole crate.
//!
//! The implementations are deliberately simple byte-at-a-time loops for now;
//! word-at-a-time and SIMD specializations are a later optimization pass.

use crate::malloc::malloc;
use crate::types::{c_char, c_int, c_void, size_t};

// --- memory block operations ----------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;
    let mut i = 0;
    while i < n {
        unsafe { *d.add(i) = *s.add(i) };
        i += 1;
    }
    dest
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;
    if (d as usize) < (s as usize) {
        // Forward copy is safe when dest precedes src.
        let mut i = 0;
        while i < n {
            unsafe { *d.add(i) = *s.add(i) };
            i += 1;
        }
    } else {
        // Copy backward to handle overlap where dest follows src.
        let mut i = n;
        while i > 0 {
            i -= 1;
            unsafe { *d.add(i) = *s.add(i) };
        }
    }
    dest
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    let d = dest as *mut u8;
    let b = c as u8;
    let mut i = 0;
    while i < n {
        unsafe { *d.add(i) = b };
        i += 1;
    }
    dest
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int {
    let pa = a as *const u8;
    let pb = b as *const u8;
    let mut i = 0;
    while i < n {
        let (x, y) = unsafe { (*pa.add(i), *pb.add(i)) };
        if x != y {
            return x as c_int - y as c_int;
        }
        i += 1;
    }
    0
}

/// `bcmp` — byte comparison returning zero/non-zero only (legacy BSD). The Rust
/// `core` slice-equality code lowers to this symbol, so we must provide it.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn bcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int {
    unsafe { memcmp(a, b, n) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void {
    let p = s as *const u8;
    let b = c as u8;
    let mut i = 0;
    while i < n {
        if unsafe { *p.add(i) } == b {
            return unsafe { p.add(i) as *mut c_void };
        }
        i += 1;
    }
    core::ptr::null_mut()
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn memrchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void {
    let p = s as *const u8;
    let b = c as u8;
    let mut i = n;
    while i > 0 {
        i -= 1;
        if unsafe { *p.add(i) } == b {
            return unsafe { p.add(i) as *mut c_void };
        }
    }
    core::ptr::null_mut()
}

// --- string length ---------------------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strlen(s: *const c_char) -> size_t {
    let mut n = 0;
    while unsafe { *s.add(n) } != 0 {
        n += 1;
    }
    n
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strnlen(s: *const c_char, maxlen: size_t) -> size_t {
    let mut n = 0;
    while n < maxlen && unsafe { *s.add(n) } != 0 {
        n += 1;
    }
    n
}

// --- string comparison -----------------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strcmp(a: *const c_char, b: *const c_char) -> c_int {
    let mut i = 0;
    loop {
        let (x, y) = unsafe { (*a.add(i), *b.add(i)) };
        if x != y || x == 0 {
            return (x as u8) as c_int - (y as u8) as c_int;
        }
        i += 1;
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int {
    let mut i = 0;
    while i < n {
        let (x, y) = unsafe { (*a.add(i), *b.add(i)) };
        if x != y || x == 0 {
            return (x as u8) as c_int - (y as u8) as c_int;
        }
        i += 1;
    }
    0
}

// --- string copy / concatenate --------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
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

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    let mut i = 0;
    // Copy up to n bytes from src, stopping at its terminator.
    while i < n && unsafe { *src.add(i) } != 0 {
        unsafe { *dest.add(i) = *src.add(i) };
        i += 1;
    }
    // Pad the remainder with NULs (strncpy's defining, oft-surprising behavior).
    while i < n {
        unsafe { *dest.add(i) = 0 };
        i += 1;
    }
    dest
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    let start = unsafe { strlen(dest) };
    let mut i = 0;
    loop {
        let ch = unsafe { *src.add(i) };
        unsafe { *dest.add(start + i) = ch };
        if ch == 0 {
            break;
        }
        i += 1;
    }
    dest
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strncat(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    let start = unsafe { strlen(dest) };
    let mut i = 0;
    while i < n && unsafe { *src.add(i) } != 0 {
        unsafe { *dest.add(start + i) = *src.add(i) };
        i += 1;
    }
    unsafe { *dest.add(start + i) = 0 };
    dest
}

// --- string search ---------------------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strchr(s: *const c_char, c: c_int) -> *mut c_char {
    let target = c as c_char;
    let mut i = 0;
    loop {
        let ch = unsafe { *s.add(i) };
        if ch == target {
            return unsafe { s.add(i) as *mut c_char };
        }
        if ch == 0 {
            return core::ptr::null_mut();
        }
        i += 1;
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strrchr(s: *const c_char, c: c_int) -> *mut c_char {
    let target = c as c_char;
    let mut last = core::ptr::null_mut();
    let mut i = 0;
    loop {
        let ch = unsafe { *s.add(i) };
        if ch == target {
            last = unsafe { s.add(i) as *mut c_char };
        }
        if ch == 0 {
            return last;
        }
        i += 1;
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char {
    if unsafe { *needle } == 0 {
        return haystack as *mut c_char;
    }
    let nlen = unsafe { strlen(needle) };
    let mut i = 0;
    loop {
        let ch = unsafe { *haystack.add(i) };
        if ch == 0 {
            return core::ptr::null_mut();
        }
        if unsafe { strncmp(haystack.add(i), needle, nlen) } == 0 {
            return unsafe { haystack.add(i) as *mut c_char };
        }
        i += 1;
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strspn(s: *const c_char, accept: *const c_char) -> size_t {
    let mut count = 0;
    loop {
        let ch = unsafe { *s.add(count) };
        if ch == 0 || unsafe { strchr(accept, ch as c_int) }.is_null() {
            return count;
        }
        count += 1;
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strcspn(s: *const c_char, reject: *const c_char) -> size_t {
    let mut count = 0;
    loop {
        let ch = unsafe { *s.add(count) };
        if ch == 0 {
            return count;
        }
        // strchr matches the terminator too; guard so it doesn't stop us early.
        if ch != 0 && !unsafe { strchr(reject, ch as c_int) }.is_null() {
            return count;
        }
        count += 1;
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char {
    let mut i = 0;
    loop {
        let ch = unsafe { *s.add(i) };
        if ch == 0 {
            return core::ptr::null_mut();
        }
        if !unsafe { strchr(accept, ch as c_int) }.is_null() {
            return unsafe { s.add(i) as *mut c_char };
        }
        i += 1;
    }
}

// --- duplication (allocating) ---------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strdup(s: *const c_char) -> *mut c_char {
    let len = unsafe { strlen(s) };
    let mem = unsafe { malloc(len + 1) } as *mut c_char;
    if mem.is_null() {
        return core::ptr::null_mut();
    }
    unsafe { memcpy(mem as *mut c_void, s as *const c_void, len + 1) };
    mem
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strndup(s: *const c_char, n: size_t) -> *mut c_char {
    let len = unsafe { strnlen(s, n) };
    let mem = unsafe { malloc(len + 1) } as *mut c_char;
    if mem.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        memcpy(mem as *mut c_void, s as *const c_void, len);
        *mem.add(len) = 0;
    }
    mem
}

// --- error strings ---------------------------------------------------------

/// `strerror` — map an errno to a static message.
///
/// Returns a pointer into a static table; the caller must not modify or free
/// it. Only a subset is named so far; unknown codes yield "Unknown error".
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn strerror(errnum: c_int) -> *mut c_char {
    use crate::errno::*;
    let msg: &[u8] = match errnum {
        EPERM => b"Operation not permitted\0",
        ENOENT => b"No such file or directory\0",
        ESRCH => b"No such process\0",
        EINTR => b"Interrupted system call\0",
        EIO => b"Input/output error\0",
        EBADF => b"Bad file descriptor\0",
        EAGAIN => b"Resource temporarily unavailable\0",
        ENOMEM => b"Cannot allocate memory\0",
        EACCES => b"Permission denied\0",
        EFAULT => b"Bad address\0",
        EBUSY => b"Device or resource busy\0",
        EEXIST => b"File exists\0",
        ENOTDIR => b"Not a directory\0",
        EISDIR => b"Is a directory\0",
        EINVAL => b"Invalid argument\0",
        EMFILE => b"Too many open files\0",
        ENOSPC => b"No space left on device\0",
        EPIPE => b"Broken pipe\0",
        ERANGE => b"Numerical result out of range\0",
        ENOSYS => b"Function not implemented\0",
        _ => b"Unknown error\0",
    };
    msg.as_ptr() as *mut c_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strlen_and_cmp() {
        let a = b"hello\0";
        let b = b"help\0";
        unsafe {
            assert_eq!(strlen(a.as_ptr() as *const c_char), 5);
            assert!(strcmp(a.as_ptr() as *const c_char, b.as_ptr() as *const c_char) < 0);
            assert_eq!(strncmp(a.as_ptr() as *const c_char, b.as_ptr() as *const c_char, 3), 0);
        }
    }

    #[test]
    fn memmove_overlap() {
        let mut buf = *b"abcdef";
        unsafe {
            // Copy "abc" (buf[0..3]) into buf[1..4]; overlapping, dest > src, so
            // memmove must copy backward. Result: a|abc|ef == "aabcef".
            memmove(
                buf.as_mut_ptr().add(1) as *mut c_void,
                buf.as_ptr() as *const c_void,
                3,
            );
        }
        assert_eq!(&buf, b"aabcef");
    }

    #[test]
    fn strchr_strstr() {
        let s = b"a needle here\0";
        let n = b"needle\0";
        unsafe {
            assert!(!strstr(s.as_ptr() as *const c_char, n.as_ptr() as *const c_char).is_null());
            assert_eq!(
                strchr(s.as_ptr() as *const c_char, b'n' as c_int),
                s.as_ptr().add(2) as *mut c_char,
            );
        }
    }
}
