/* inttypes.h — format conversion of integer types (C standard). Part of rustlibc. */
#ifndef _RUSTLIBC_INTTYPES_H
#define _RUSTLIBC_INTTYPES_H

#include <stdint.h>

/* Length modifiers for the fixed-width types (LP64: 64-bit uses "l"). */
#define PRId8 "d"
#define PRId16 "d"
#define PRId32 "d"
#define PRId64 "ld"
#define PRIu8 "u"
#define PRIu16 "u"
#define PRIu32 "u"
#define PRIu64 "lu"
#define PRIx8 "x"
#define PRIx16 "x"
#define PRIx32 "x"
#define PRIx64 "lx"
#define PRIdPTR "ld"
#define PRIuPTR "lu"
#define PRIxPTR "lx"

typedef struct {
	intmax_t quot;
	intmax_t rem;
} imaxdiv_t;

#ifdef __cplusplus
extern "C" {
#endif

intmax_t imaxabs(intmax_t j);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_INTTYPES_H */
