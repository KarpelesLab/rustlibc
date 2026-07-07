//! `<signal.h>` — signal numbers and (eventually) disposition management.
//!
//! Signal *numbers* and `kill`/`raise` are real. `sigaction`/`signal` handler
//! installation is **stubbed**: correct delivery needs a signal trampoline
//! (`rt_sigreturn`) and `sa_restorer` wiring per arch, which is a focused later
//! effort. Marked `STUB`.

use crate::platform::linux::{nr, syscall2};
use crate::types::{c_int, pid_t};

// Common signal numbers (Linux, identical on x86_64 and aarch64).
pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGILL: c_int = 4;
pub const SIGTRAP: c_int = 5;
pub const SIGABRT: c_int = 6;
pub const SIGBUS: c_int = 7;
pub const SIGFPE: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGUSR1: c_int = 10;
pub const SIGSEGV: c_int = 11;
pub const SIGUSR2: c_int = 12;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGTERM: c_int = 15;
pub const SIGCHLD: c_int = 17;
pub const SIGCONT: c_int = 18;
pub const SIGSTOP: c_int = 19;

/// Handler sentinels for `signal()`.
pub const SIG_DFL: usize = 0;
pub const SIG_IGN: usize = 1;
pub const SIG_ERR: isize = -1;

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn kill(pid: pid_t, sig: c_int) -> c_int {
    unsafe { syscall2(nr::KILL, pid as usize, sig as usize) as c_int }
}

/// `raise` — send a signal to the current process.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn raise(sig: c_int) -> c_int {
    let pid = crate::unistd::getpid();
    unsafe { kill(pid, sig) }
}

/// C `signal()` handler type.
pub type sighandler_t = usize;

/// `signal` — STUB. Installing handlers needs the arch signal trampoline.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn signal(_signum: c_int, _handler: sighandler_t) -> sighandler_t {
    // STUB
    crate::errno::set_errno(crate::errno::ENOSYS);
    SIG_ERR as usize
}
