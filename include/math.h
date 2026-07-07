/* math.h — floating-point math (C standard).
 *
 * Part of rustlibc. Sign/compare/classify and sqrt are implemented; the
 * transcendental functions are STUBs pending a real libm (they set errno =
 * ENOSYS and return NaN). See src/math.rs. */
#ifndef _RUSTLIBC_MATH_H
#define _RUSTLIBC_MATH_H

#ifdef __cplusplus
extern "C" {
#endif

#define M_PI 3.14159265358979323846
#define M_E 2.7182818284590452354
#define HUGE_VAL __builtin_huge_val()
#define HUGE_VALF __builtin_huge_valf()
#define INFINITY __builtin_inff()
#define NAN __builtin_nanf("")

/* Implemented. */
double fabs(double x);
float fabsf(float x);
double copysign(double x, double y);
float copysignf(float x, float y);
double fmax(double a, double b);
double fmin(double a, double b);
double sqrt(double x);
double fmod(double x, double y);

/* STUB (return NaN, set errno = ENOSYS). */
double sin(double x);
double cos(double x);
double tan(double x);
double asin(double x);
double acos(double x);
double atan(double x);
double sinh(double x);
double cosh(double x);
double tanh(double x);
double exp(double x);
double log(double x);
double log2(double x);
double log10(double x);
double cbrt(double x);
double pow(double x, double y);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_MATH_H */
