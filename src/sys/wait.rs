//! `<sys/wait.h>` — process wait.
//!
//! Real wrappers over `wait4`. The `W*` status-inspection macros are pure bit
//! math and live in the C header.

use crate::platform::linux::{nr, syscall4};
use crate::types::{c_int, pid_t};

pub const WNOHANG: c_int = 1;
pub const WUNTRACED: c_int = 2;
pub const WCONTINUED: c_int = 8;

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t {
    // wait4(pid, status, options, rusage = NULL)
    unsafe { syscall4(nr::WAIT4, pid as usize, status as usize, options as usize, 0) as pid_t }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn wait(status: *mut c_int) -> pid_t {
    unsafe { waitpid(-1, status, 0) }
}
