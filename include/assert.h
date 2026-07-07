/* assert.h — diagnostics (C standard).
 *
 * Part of rustlibc. Note: <assert.h> is intentionally not guarded against
 * multiple inclusion, because assert's definition depends on NDEBUG at each
 * point of inclusion. */
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

_Noreturn void __assert_fail(const char *expr, const char *file, int line,
                             const char *func);

#ifdef __cplusplus
}
#endif

#undef assert
#ifdef NDEBUG
#define assert(expr) ((void)0)
#else
#define assert(expr) \
	((expr) ? (void)0 : __assert_fail(#expr, __FILE__, __LINE__, __func__))
#endif
