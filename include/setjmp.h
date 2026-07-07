/* setjmp.h — non-local jumps (C standard).
 *
 * Part of rustlibc. setjmp/longjmp are implemented (naked asm). The signal
 * mask is not saved yet (these behave as _setjmp). See src/setjmp.rs. */
#ifndef _RUSTLIBC_SETJMP_H
#define _RUSTLIBC_SETJMP_H

#ifdef __cplusplus
extern "C" {
#endif

/* Matches struct __jmp_buf_tag in src/setjmp.rs (256 bytes). jmp_buf is an
 * array type so it decays to a pointer when passed to setjmp/longjmp. */
typedef struct __jmp_buf_tag {
	unsigned long long __data[32];
} jmp_buf[1];

int setjmp(jmp_buf env);
_Noreturn void longjmp(jmp_buf env, int val);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_SETJMP_H */
