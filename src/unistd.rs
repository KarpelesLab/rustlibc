//! `<unistd.h>` — core POSIX system-call wrappers.
//!
//! Thin translations from the C ABI to the Linux syscall layer. Real, not
//! stubbed, for the syscalls listed; the surface will grow over time.

use crate::platform::linux::{nr, syscall1, syscall3};
use crate::types::{c_char, c_int, c_uint, off_t, pid_t, size_t, ssize_t};

// Standard file descriptors.
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

// `lseek` whence values.
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn read(fd: c_int, buf: *mut core::ffi::c_void, count: size_t) -> ssize_t {
    unsafe { syscall3(nr::READ, fd as usize, buf as usize, count) as ssize_t }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn write(fd: c_int, buf: *const core::ffi::c_void, count: size_t) -> ssize_t {
    unsafe { syscall3(nr::WRITE, fd as usize, buf as usize, count) as ssize_t }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn close(fd: c_int) -> c_int {
    unsafe { syscall1(nr::CLOSE, fd as usize) as c_int }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    unsafe { syscall3(nr::LSEEK, fd as usize, offset as usize, whence as usize) as off_t }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn getpid() -> pid_t {
    unsafe { crate::platform::linux::syscall0(nr::GETPID) as pid_t }
}

/// `_exit` — terminate the process immediately without running atexit handlers
/// or flushing stdio. Uses `exit_group` so all threads die.
///
/// Marked `-> !`: the syscall never returns. The trailing loop is only there to
/// satisfy the type in the (impossible) case the kernel returns.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn _exit(status: c_int) -> ! {
    unsafe {
        crate::platform::linux::syscall1(nr::EXIT_GROUP, status as usize);
        // Fallback: single-thread exit, then spin.
        crate::platform::linux::syscall1(nr::EXIT, status as usize);
    }
    // Unreachable: the kernel never returns from exit_group/exit.
    loop {
        core::hint::spin_loop();
    }
}

/// `usleep` — suspend for `usec` microseconds via `clock_nanosleep`.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn usleep(usec: c_uint) -> c_int {
    use crate::types::timespec;
    let ts = timespec {
        tv_sec: (usec / 1_000_000) as _,
        tv_nsec: ((usec % 1_000_000) * 1000) as _,
    };
    // CLOCK_REALTIME=0, flags=0, rqtp=&ts, rmtp=NULL
    unsafe {
        crate::platform::linux::syscall4(
            nr::CLOCK_NANOSLEEP,
            0,
            0,
            &ts as *const timespec as usize,
            0,
        ) as c_int
    }
}

/// `sleep` — suspend for `seconds`. Returns 0 on full completion (we do not yet
/// report remaining time on interruption).
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn sleep(seconds: c_uint) -> c_uint {
    use crate::types::timespec;
    let ts = timespec {
        tv_sec: seconds as _,
        tv_nsec: 0,
    };
    unsafe {
        crate::platform::linux::syscall4(
            nr::CLOCK_NANOSLEEP,
            0,
            0,
            &ts as *const timespec as usize,
            0,
        );
    }
    0
}

/// `getcwd` — copy the current working directory into `buf`.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char {
    let ret = unsafe { crate::platform::linux::syscall2(nr::GETCWD, buf as usize, size) };
    if ret < 0 {
        core::ptr::null_mut()
    } else {
        buf
    }
}
