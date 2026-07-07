//! `<ctype.h>` — single-byte character classification and conversion.
//!
//! All functions operate on an `int` that must be representable as `unsigned
//! char` or equal `EOF`; behavior is defined for the "C" locale only, which is
//! all we currently support. Fully implemented — pure computation.

use crate::types::c_int;

#[inline]
fn as_byte(c: c_int) -> Option<u8> {
    // Valid inputs are EOF (-1) or 0..=255. For classification we only act on
    // the 0..=255 range and treat everything else as "not a member".
    if (0..=255).contains(&c) {
        Some(c as u8)
    } else {
        None
    }
}

macro_rules! ctype_predicate {
    ($name:ident, $b:ident => $body:expr) => {
        #[cfg_attr(not(test), unsafe(no_mangle))]
        pub extern "C" fn $name(c: c_int) -> c_int {
            match as_byte(c) {
                Some($b) => ($body) as c_int,
                None => 0,
            }
        }
    };
}

ctype_predicate!(isalpha, b => b.is_ascii_alphabetic());
ctype_predicate!(isdigit, b => b.is_ascii_digit());
ctype_predicate!(isalnum, b => b.is_ascii_alphanumeric());
ctype_predicate!(isspace, b => matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c));
ctype_predicate!(isupper, b => b.is_ascii_uppercase());
ctype_predicate!(islower, b => b.is_ascii_lowercase());
ctype_predicate!(isxdigit, b => b.is_ascii_hexdigit());
ctype_predicate!(iscntrl, b => b.is_ascii_control());
ctype_predicate!(isgraph, b => b.is_ascii_graphic());
ctype_predicate!(isprint, b => (0x20..=0x7e).contains(&b));
ctype_predicate!(ispunct, b => b.is_ascii_punctuation());
ctype_predicate!(isblank, b => matches!(b, b' ' | b'\t'));

/// `isascii` (POSIX/XSI) — true for 0..=127. Defined for all `int`.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn isascii(c: c_int) -> c_int {
    (c & !0x7f == 0) as c_int
}

/// `toascii` (POSIX/XSI) — strip to 7 bits.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn toascii(c: c_int) -> c_int {
    c & 0x7f
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn tolower(c: c_int) -> c_int {
    match as_byte(c) {
        Some(b) if b.is_ascii_uppercase() => (b + 32) as c_int,
        _ => c,
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn toupper(c: c_int) -> c_int {
    match as_byte(c) {
        Some(b) if b.is_ascii_lowercase() => (b - 32) as c_int,
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification() {
        assert_ne!(isalpha(b'a' as c_int), 0);
        assert_eq!(isalpha(b'1' as c_int), 0);
        assert_ne!(isdigit(b'7' as c_int), 0);
        assert_ne!(isspace(b'\n' as c_int), 0);
        assert_eq!(isspace(b'x' as c_int), 0);
        assert_eq!(isalpha(-1), 0); // EOF
    }

    #[test]
    fn conversion() {
        assert_eq!(tolower(b'A' as c_int), b'a' as c_int);
        assert_eq!(toupper(b'z' as c_int), b'Z' as c_int);
        assert_eq!(tolower(b'5' as c_int), b'5' as c_int);
    }
}
