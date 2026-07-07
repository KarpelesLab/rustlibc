/* ctype.h — character classification and conversion (C standard).
 *
 * Part of rustlibc. All declarations here are implemented (C locale). */
#ifndef _RUSTLIBC_CTYPE_H
#define _RUSTLIBC_CTYPE_H

#ifdef __cplusplus
extern "C" {
#endif

int isalpha(int c);
int isdigit(int c);
int isalnum(int c);
int isspace(int c);
int isupper(int c);
int islower(int c);
int isxdigit(int c);
int iscntrl(int c);
int isgraph(int c);
int isprint(int c);
int ispunct(int c);
int isblank(int c);
int isascii(int c);
int toascii(int c);
int tolower(int c);
int toupper(int c);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_CTYPE_H */
