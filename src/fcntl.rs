//! `<fcntl.h>` — file control: `open`/`openat` flags and wrappers.
//!
//! `open`/`openat` are real (routed through `openat`, which both target arches
//! provide natively). `fcntl` itself is a thin passthrough.

use crate::platform::linux::{nr, syscall3, syscall4};
use crate::types::{c_char, c_int, mode_t};

// Access modes.
pub const O_RDONLY: c_int = 0o0;
pub const O_WRONLY: c_int = 0o1;
pub const O_RDWR: c_int = 0o2;
// Creation / status flags (x86_64/aarch64 values).
pub const O_CREAT: c_int = 0o100;
pub const O_EXCL: c_int = 0o200;
pub const O_NOCTTY: c_int = 0o400;
pub const O_TRUNC: c_int = 0o1000;
pub const O_APPEND: c_int = 0o2000;
pub const O_NONBLOCK: c_int = 0o4000;
pub const O_DIRECTORY: c_int = 0o200000;
pub const O_CLOEXEC: c_int = 0o2000000;

/// `AT_FDCWD` — interpret a relative path against the current directory.
pub const AT_FDCWD: c_int = -100;

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn openat(
    dirfd: c_int,
    pathname: *const c_char,
    flags: c_int,
    mode: mode_t,
) -> c_int {
    unsafe {
        syscall4(
            nr::OPENAT,
            dirfd as usize,
            pathname as usize,
            flags as usize,
            mode as usize,
        ) as c_int
    }
}

/// `open` — implemented via `openat(AT_FDCWD, ...)` so it works uniformly on
/// arches (like aarch64) that lack a bare `open` syscall.
///
/// C declares `open` as variadic (`mode` is only read when `O_CREAT`/`O_TMPFILE`
/// is set). Without `c_variadic` we cannot read the optional `mode` argument, so
/// this fixed-arity form passes `mode = 0`. Creating files with a specific mode
/// currently requires calling [`openat`] directly. TODO(variadic).
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn open(pathname: *const c_char, flags: c_int) -> c_int {
    unsafe { openat(AT_FDCWD, pathname, flags, 0) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fcntl(fd: c_int, cmd: c_int, arg: c_int) -> c_int {
    // Also variadic in C; the third arg is command-dependent. We accept a single
    // `int`/pointer-sized arg, which covers the common F_GETFL/F_SETFL/F_DUPFD.
    unsafe { syscall3(nr::FCNTL, fd as usize, cmd as usize, arg as usize) as c_int }
}
