/* stdlib.h — general utilities (C standard).
 *
 * Part of rustlibc. Conversion, allocation, search/sort and process control
 * are implemented; getenv is a stub (see notes in src/stdlib.rs). */
#ifndef _RUSTLIBC_STDLIB_H
#define _RUSTLIBC_STDLIB_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1
#define RAND_MAX 0x7fffffff

/* Allocation (src/malloc.rs). */
void *malloc(size_t size);
void free(void *ptr);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void *aligned_alloc(size_t alignment, size_t size);

/* String -> number. */
long strtol(const char *nptr, char **endptr, int base);
long long strtoll(const char *nptr, char **endptr, int base);
unsigned long strtoul(const char *nptr, char **endptr, int base);
int atoi(const char *nptr);
long atol(const char *nptr);
long long atoll(const char *nptr);

/* Integer arithmetic. */
int abs(int j);
long labs(long j);

/* Search & sort. */
void *bsearch(const void *key, const void *base, size_t nmemb, size_t size,
              int (*compar)(const void *, const void *));
void qsort(void *base, size_t nmemb, size_t size,
           int (*compar)(const void *, const void *));

/* Process control & environment. */
_Noreturn void exit(int status);
_Noreturn void abort(void);
char *getenv(const char *name);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_STDLIB_H */
