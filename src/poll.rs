//! `<poll.h>` — synchronous I/O multiplexing.
//!
//! Real `poll`. On x86_64 this is the `poll` syscall; aarch64 has only `ppoll`,
//! so there we translate the millisecond timeout into a `timespec` and call
//! `ppoll` with no signal mask.

use crate::platform::linux::nr;
use crate::types::{c_int, c_short, c_ulong};

pub type nfds_t = c_ulong;

pub const POLLIN: c_short = 0x001;
pub const POLLPRI: c_short = 0x002;
pub const POLLOUT: c_short = 0x004;
pub const POLLERR: c_short = 0x008;
pub const POLLHUP: c_short = 0x010;
pub const POLLNVAL: c_short = 0x020;

/// `struct pollfd`.
#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[cfg(target_arch = "x86_64")]
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int {
    unsafe {
        crate::platform::linux::syscall3(nr::POLL, fds as usize, nfds as usize, timeout as usize)
            as c_int
    }
}

#[cfg(target_arch = "aarch64")]
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int {
    use crate::types::timespec;
    // Negative timeout means "block indefinitely" -> NULL timespec.
    let ts = timespec {
        tv_sec: (timeout / 1000) as _,
        tv_nsec: ((timeout % 1000) * 1_000_000) as _,
    };
    let tsp: usize = if timeout < 0 {
        0
    } else {
        &ts as *const timespec as usize
    };
    unsafe {
        // ppoll(fds, nfds, tsp, sigmask = NULL, sigsetsize = 0)
        crate::platform::linux::syscall5(nr::PPOLL, fds as usize, nfds as usize, tsp, 0, 0) as c_int
    }
}
