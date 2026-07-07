/* stddef.h — common definitions (C standard).
 *
 * Part of rustlibc. These are the freestanding basics; a conforming compiler
 * may also provide its own <stddef.h>, but we ship one so rustlibc is
 * self-contained. */
#ifndef _RUSTLIBC_STDDEF_H
#define _RUSTLIBC_STDDEF_H

#ifndef NULL
#define NULL ((void *)0)
#endif

#define offsetof(type, member) __builtin_offsetof(type, member)

typedef __SIZE_TYPE__ size_t;
typedef __PTRDIFF_TYPE__ ptrdiff_t;
typedef __WCHAR_TYPE__ wchar_t;

/* Maximum-alignment type (C11). */
typedef struct {
	long long __ll;
	long double __ld;
} max_align_t;

#endif /* _RUSTLIBC_STDDEF_H */
