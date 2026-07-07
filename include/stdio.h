/* stdio.h — standard input/output (C standard).
 *
 * Part of rustlibc. The non-variadic surface (fopen excepted) and the full
 * printf family are implemented; the scanf family is not yet provided. See
 * src/stdio.rs. */
#ifndef _RUSTLIBC_STDIO_H
#define _RUSTLIBC_STDIO_H

#include <stdarg.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#define EOF (-1)
#define BUFSIZ 8192

typedef struct _rustlibc_FILE FILE;

extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;

/* Output. */
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream);
int fputs(const char *s, FILE *stream);
int fputc(int c, FILE *stream);
int putc(int c, FILE *stream);
int putchar(int c);
int puts(const char *s);
int fflush(FILE *stream);

/* Input. */
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream);

/* Stream management. */
FILE *fopen(const char *path, const char *mode);
int fclose(FILE *stream);
int fileno(FILE *stream);
int ferror(FILE *stream);
int feof(FILE *stream);
void clearerr(FILE *stream);
void perror(const char *s);

/* Formatted output. */
int printf(const char *format, ...);
int fprintf(FILE *stream, const char *format, ...);
int snprintf(char *str, size_t size, const char *format, ...);
int sprintf(char *str, const char *format, ...);
int vprintf(const char *format, va_list ap);
int vfprintf(FILE *stream, const char *format, va_list ap);
int vsnprintf(char *str, size_t size, const char *format, va_list ap);
int vsprintf(char *str, const char *format, va_list ap);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_STDIO_H */
