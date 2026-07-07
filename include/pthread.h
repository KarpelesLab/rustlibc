/* pthread.h — POSIX threads. Part of rustlibc.
 *
 * Threads are not implemented yet: pthread_create returns ENOSYS, while the
 * mutex/once primitives are working no-ops (valid single-threaded). The opaque
 * types match src/pthread.rs. */
#ifndef _RUSTLIBC_PTHREAD_H
#define _RUSTLIBC_PTHREAD_H

#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef unsigned long pthread_t;

typedef struct {
	int __state;
} pthread_mutex_t;

typedef struct {
	int __dummy;
} pthread_mutexattr_t;

typedef struct {
	int __done;
} pthread_once_t;

#define PTHREAD_MUTEX_INITIALIZER {0}
#define PTHREAD_ONCE_INIT {0}

int pthread_create(pthread_t *thread, const void *attr,
                   void *(*start_routine)(void *), void *arg);
int pthread_join(pthread_t thread, void **retval);
pthread_t pthread_self(void);

int pthread_mutex_init(pthread_mutex_t *mutex, const pthread_mutexattr_t *attr);
int pthread_mutex_lock(pthread_mutex_t *mutex);
int pthread_mutex_trylock(pthread_mutex_t *mutex);
int pthread_mutex_unlock(pthread_mutex_t *mutex);
int pthread_mutex_destroy(pthread_mutex_t *mutex);

int pthread_once(pthread_once_t *once_control, void (*init_routine)(void));

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_PTHREAD_H */
