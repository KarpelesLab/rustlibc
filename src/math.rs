//! `<math.h>` — floating-point math.
//!
//! Exact, cheap operations (sign/compare/classify) are implemented directly.
//! The transcendental functions (`sin`, `exp`, `pow`, ...) require a real libm
//! and are **stubbed** for now — they set `errno = ENOSYS` where a domain makes
//! sense and return a sentinel. Marked `STUB`; porting a `libm` is a dedicated
//! later effort.

use crate::types::{c_double, c_float, c_int};

pub const M_PI: c_double = core::f64::consts::PI;
pub const M_E: c_double = core::f64::consts::E;
pub const HUGE_VAL: c_double = f64::INFINITY;
pub const HUGE_VALF: c_float = f32::INFINITY;
pub const INFINITY: c_float = f32::INFINITY;
pub const NAN: c_float = f32::NAN;

// --- sign, comparison, classification (exact) ------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn fabs(x: c_double) -> c_double {
    f64::from_bits(x.to_bits() & !(1u64 << 63))
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn fabsf(x: c_float) -> c_float {
    f32::from_bits(x.to_bits() & !(1u32 << 31))
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn copysign(x: c_double, y: c_double) -> c_double {
    let sign = y.to_bits() & (1u64 << 63);
    f64::from_bits((x.to_bits() & !(1u64 << 63)) | sign)
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn copysignf(x: c_float, y: c_float) -> c_float {
    let sign = y.to_bits() & (1u32 << 31);
    f32::from_bits((x.to_bits() & !(1u32 << 31)) | sign)
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn fmax(a: c_double, b: c_double) -> c_double {
    // C fmax: if one operand is NaN, return the other; otherwise the larger.
    if a.is_nan() {
        return b;
    }
    if b.is_nan() {
        return a;
    }
    if a > b { a } else { b }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn fmin(a: c_double, b: c_double) -> c_double {
    if a.is_nan() {
        return b;
    }
    if b.is_nan() {
        return a;
    }
    if a < b { a } else { b }
}

/// `isnan` is a type-generic macro in C; the ABI helper glibc emits is
/// `__isnan`. Provide both spellings for `double`.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn __isnan(x: c_double) -> c_int {
    x.is_nan() as c_int
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn __isinf(x: c_double) -> c_int {
    if x.is_infinite() {
        if x > 0.0 { 1 } else { -1 }
    } else {
        0
    }
}

/// `sqrt` — Newton–Raphson refinement. Not stubbed: exact enough for general
/// use and needs no external libm. Handles the special values explicitly.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn sqrt(x: c_double) -> c_double {
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 || x.is_nan() || x.is_infinite() {
        return x;
    }
    // Seed from the exponent halved, then iterate. Converges quadratically.
    let mut guess = f64::from_bits((x.to_bits() >> 1) + (1023u64 << 51));
    let mut i = 0;
    while i < 8 {
        guess = 0.5 * (guess + x / guess);
        i += 1;
    }
    guess
}

// --- transcendentals: STUB -------------------------------------------------

macro_rules! math_stub {
    ($($name:ident),* $(,)?) => {$(
        #[cfg_attr(not(test), unsafe(no_mangle))]
        pub extern "C" fn $name(_x: c_double) -> c_double {
            // STUB: needs a real libm implementation.
            crate::errno::set_errno(crate::errno::ENOSYS);
            f64::NAN
        }
    )*};
}

math_stub!(sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, exp, log, log2, log10, cbrt);

/// `pow` — STUB (two-argument). See [`math_stub`] for the one-argument family.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn pow(_x: c_double, _y: c_double) -> c_double {
    // STUB
    crate::errno::set_errno(crate::errno::ENOSYS);
    f64::NAN
}

/// `fmod` — remainder of `x/y`. Uses the exact core operator.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn fmod(x: c_double, y: c_double) -> c_double {
    x % y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_ops() {
        assert_eq!(fabs(-3.5), 3.5);
        assert_eq!(copysign(2.0, -1.0), -2.0);
        assert_eq!(fmax(1.0, 2.0), 2.0);
    }

    #[test]
    fn sqrt_ok() {
        assert!((sqrt(16.0) - 4.0).abs() < 1e-9);
        assert!((sqrt(2.0) - 1.414_213_562).abs() < 1e-6);
        assert!(sqrt(-1.0).is_nan());
    }
}
