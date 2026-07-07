/* locale.h — localization (C standard). Part of rustlibc.
 *
 * Only the "C"/"POSIX" locale is supported; setlocale is a stub. */
#ifndef _RUSTLIBC_LOCALE_H
#define _RUSTLIBC_LOCALE_H

#ifdef __cplusplus
extern "C" {
#endif

#define LC_CTYPE 0
#define LC_NUMERIC 1
#define LC_TIME 2
#define LC_COLLATE 3
#define LC_MONETARY 4
#define LC_ALL 6

char *setlocale(int category, const char *locale);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_LOCALE_H */
