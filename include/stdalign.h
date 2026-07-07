/* stdalign.h — alignment (C standard, pre-C23). Part of rustlibc. */
#ifndef _RUSTLIBC_STDALIGN_H
#define _RUSTLIBC_STDALIGN_H

#if !defined(__cplusplus)
#define alignas _Alignas
#define alignof _Alignof
#endif

#define __alignas_is_defined 1
#define __alignof_is_defined 1

#endif /* _RUSTLIBC_STDALIGN_H */
