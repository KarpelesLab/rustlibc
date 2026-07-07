/* strings.h — BSD/POSIX case-insensitive and bit helpers.
 *
 * Part of rustlibc. All declarations here are implemented. */
#ifndef _RUSTLIBC_STRINGS_H
#define _RUSTLIBC_STRINGS_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

int strcasecmp(const char *a, const char *b);
int strncasecmp(const char *a, const char *b, size_t n);
void bzero(void *s, size_t n);
void bcopy(const void *src, void *dest, size_t n);
int ffs(int i);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_STRINGS_H */
