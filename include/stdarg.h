/* stdarg.h — variable arguments (C standard).
 *
 * Part of rustlibc. These forward to the compiler builtins. Note: rustlibc's
 * own printf/scanf cannot yet *consume* a va_list (needs `c_variadic` on the
 * pinned toolchain), but C programs may still use these macros in their own
 * variadic functions. */
#ifndef _RUSTLIBC_STDARG_H
#define _RUSTLIBC_STDARG_H

typedef __builtin_va_list va_list;

#define va_start(ap, last) __builtin_va_start(ap, last)
#define va_end(ap) __builtin_va_end(ap)
#define va_arg(ap, type) __builtin_va_arg(ap, type)
#define va_copy(dst, src) __builtin_va_copy(dst, src)

#endif /* _RUSTLIBC_STDARG_H */
