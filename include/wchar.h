/* wchar.h — wide-character strings (C standard).
 *
 * Part of rustlibc. Length/compare/copy are implemented; multibyte conversion
 * is a STUB pending a UTF-8 codec. See src/wchar.rs. */
#ifndef _RUSTLIBC_WCHAR_H
#define _RUSTLIBC_WCHAR_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef __WCHAR_TYPE__ wchar_t;
typedef struct { int __opaque; } mbstate_t;

size_t wcslen(const wchar_t *s);
int wcscmp(const wchar_t *a, const wchar_t *b);
wchar_t *wcscpy(wchar_t *dest, const wchar_t *src);
size_t mbrtowc(wchar_t *pwc, const char *s, size_t n, mbstate_t *ps);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_WCHAR_H */
