//! `<pthread.h>` — POSIX threads.
//!
//! Threads are **not implemented yet** (no `clone`-based thread creation, TLS,
//! or scheduler integration). To let single-threaded programs that merely
//! *reference* pthreads run, the mutex/once primitives are provided as working
//! no-ops (correct while the process stays single-threaded), while
//! `pthread_create` reports `ENOSYS`. Everything here is provisional — marked
//! `STUB` where the behavior is a placeholder.

use crate::types::{c_int, c_ulong, c_void};

pub type pthread_t = c_ulong;

/// Opaque mutex. A single state word is enough for the no-op implementation;
/// the C header reserves the same size.
#[repr(C)]
pub struct pthread_mutex_t {
    __state: c_int,
}

#[repr(C)]
pub struct pthread_mutexattr_t {
    __dummy: c_int,
}

#[repr(C)]
pub struct pthread_once_t {
    __done: c_int,
}

/// Thread entry signature.
pub type StartRoutine = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

/// `pthread_create` — STUB: thread creation is not implemented.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn pthread_create(
    _thread: *mut pthread_t,
    _attr: *const c_void,
    _start: StartRoutine,
    _arg: *mut c_void,
) -> c_int {
    // STUB
    crate::errno::ENOSYS
}

/// `pthread_join` — STUB (no threads exist to join).
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn pthread_join(_thread: pthread_t, _retval: *mut *mut c_void) -> c_int {
    // STUB
    crate::errno::ENOSYS
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn pthread_self() -> pthread_t {
    // Single "main" thread until real threading lands.
    1
}

// Mutexes: no-ops that succeed. Correct as long as the process is
// single-threaded; to be replaced with real futex-backed locks with threads.

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn pthread_mutex_init(
    _mutex: *mut pthread_mutex_t,
    _attr: *const pthread_mutexattr_t,
) -> c_int {
    0
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn pthread_mutex_lock(_mutex: *mut pthread_mutex_t) -> c_int {
    0
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn pthread_mutex_trylock(_mutex: *mut pthread_mutex_t) -> c_int {
    0
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn pthread_mutex_unlock(_mutex: *mut pthread_mutex_t) -> c_int {
    0
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn pthread_mutex_destroy(_mutex: *mut pthread_mutex_t) -> c_int {
    0
}

/// `pthread_once` — runs `init_routine` exactly once. Correct single-threaded.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn pthread_once(
    once_control: *mut pthread_once_t,
    init_routine: unsafe extern "C" fn(),
) -> c_int {
    if once_control.is_null() {
        return crate::errno::EINVAL;
    }
    unsafe {
        if (*once_control).__done == 0 {
            (*once_control).__done = 1;
            init_routine();
        }
    }
    0
}
