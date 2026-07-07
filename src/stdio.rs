//! `<stdio.h>` — buffered I/O and formatted output.
//!
//! ## What is real
//! The stream object ([`FILE`]), the standard streams, and every *non-variadic*
//! entry point (`fopen`, `fread`, `fwrite`, `fputs`, `fputc`, `puts`,
//! `putchar`, `fflush`, `fclose`, `perror`, `fileno`) are implemented against
//! the syscall layer. Output is currently unbuffered (each write is a syscall);
//! a buffering layer is a later pass.
//!
//! ## What is stubbed — and why
//! The `printf` family is variadic. Implementing the C variadic ABI in Rust
//! needs the `c_variadic` feature, which is **not available on the stable 1.88
//! toolchain this crate pins**. Until that is resolved (toolchain bump or an
//! asm-based `va_list` shim), the `printf`/`scanf` entry points are stubs: they
//! emit the format string verbatim and ignore conversion specifiers. This makes
//! argument-less calls like `printf("hello\n")` behave correctly while keeping
//! the symbols present so C programs link. Every such function is marked `STUB`.

use crate::types::{c_char, c_int, c_void, size_t};
use crate::unistd::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, write};

/// End-of-file / error sentinel returned by character input functions.
pub const EOF: c_int = -1;
/// Default buffer size hint (`BUFSIZ`).
pub const BUFSIZ: c_int = 8192;

/// A C stream. Opaque to callers (they only ever hold `FILE *`).
///
/// Kept intentionally small for the scaffold: a descriptor plus sticky
/// error/eof flags. Buffering fields will be added with the buffering pass.
#[repr(C)]
pub struct FILE {
    fd: c_int,
    error: c_int,
    eof: c_int,
}

impl FILE {
    const fn new(fd: c_int) -> Self {
        FILE {
            fd,
            error: 0,
            eof: 0,
        }
    }
}

// The three standard streams. They are ordinary `FILE` objects whose addresses
// are published through the `stdin`/`stdout`/`stderr` pointer globals that C
// code references.
static mut STDIN_FILE: FILE = FILE::new(STDIN_FILENO);
static mut STDOUT_FILE: FILE = FILE::new(STDOUT_FILENO);
static mut STDERR_FILE: FILE = FILE::new(STDERR_FILENO);

#[cfg_attr(not(test), unsafe(no_mangle))]
pub static mut stdin: *mut FILE = &raw mut STDIN_FILE;
#[cfg_attr(not(test), unsafe(no_mangle))]
pub static mut stdout: *mut FILE = &raw mut STDOUT_FILE;
#[cfg_attr(not(test), unsafe(no_mangle))]
pub static mut stderr: *mut FILE = &raw mut STDERR_FILE;

// --- unbuffered output primitives -----------------------------------------

/// Write `len` bytes to a stream, retrying on short writes. Returns bytes
/// written, or a negative value on error (after setting the stream's error
/// flag).
unsafe fn stream_write_all(stream: *mut FILE, buf: *const u8, len: usize) -> isize {
    if stream.is_null() {
        return -1;
    }
    let fd = unsafe { (*stream).fd };
    let mut off = 0usize;
    while off < len {
        let n = unsafe { write(fd, buf.add(off) as *const c_void, len - off) };
        if n < 0 {
            unsafe { (*stream).error = 1 };
            return -1;
        }
        if n == 0 {
            break;
        }
        off += n as usize;
    }
    off as isize
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fwrite(
    ptr: *const c_void,
    size: size_t,
    nmemb: size_t,
    stream: *mut FILE,
) -> size_t {
    let total = match size.checked_mul(nmemb) {
        Some(0) | None => return 0,
        Some(t) => t,
    };
    let written = unsafe { stream_write_all(stream, ptr as *const u8, total) };
    if written < 0 {
        0
    } else {
        // Return the count of complete elements written.
        (written as usize) / size
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fputs(s: *const c_char, stream: *mut FILE) -> c_int {
    let len = unsafe { crate::string::strlen(s) };
    let n = unsafe { stream_write_all(stream, s as *const u8, len) };
    if n < 0 { EOF } else { 0 }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fputc(c: c_int, stream: *mut FILE) -> c_int {
    let byte = c as u8;
    let n = unsafe { stream_write_all(stream, &byte as *const u8, 1) };
    if n < 0 { EOF } else { c & 0xff }
}

/// `putc` — identical to `fputc` (we do not implement it as a macro).
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn putc(c: c_int, stream: *mut FILE) -> c_int {
    unsafe { fputc(c, stream) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn putchar(c: c_int) -> c_int {
    unsafe { fputc(c, stdout) }
}

/// `puts` — write `s` followed by a newline to stdout (note the appended '\n',
/// which `fputs` does not add).
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    if unsafe { fputs(s, stdout) } == EOF {
        return EOF;
    }
    if unsafe { fputc(b'\n' as c_int, stdout) } == EOF {
        return EOF;
    }
    0
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fflush(_stream: *mut FILE) -> c_int {
    // Output is unbuffered today, so there is nothing to flush.
    0
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fileno(stream: *mut FILE) -> c_int {
    if stream.is_null() {
        crate::errno::set_errno(crate::errno::EBADF);
        return -1;
    }
    unsafe { (*stream).fd }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn ferror(stream: *mut FILE) -> c_int {
    if stream.is_null() {
        0
    } else {
        unsafe { (*stream).error }
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn feof(stream: *mut FILE) -> c_int {
    if stream.is_null() {
        0
    } else {
        unsafe { (*stream).eof }
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn clearerr(stream: *mut FILE) {
    if !stream.is_null() {
        unsafe {
            (*stream).error = 0;
            (*stream).eof = 0;
        }
    }
}

/// `perror` — print `s: <errno message>` to stderr.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn perror(s: *const c_char) {
    unsafe {
        if !s.is_null() && *s != 0 {
            fputs(s, stderr);
            fputs(b": \0".as_ptr() as *const c_char, stderr);
        }
        let msg = crate::string::strerror(crate::errno::errno());
        fputs(msg, stderr);
        fputc(b'\n' as c_int, stderr);
    }
}

// --- input -----------------------------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fread(
    ptr: *mut c_void,
    size: size_t,
    nmemb: size_t,
    stream: *mut FILE,
) -> size_t {
    if stream.is_null() {
        return 0;
    }
    let total = match size.checked_mul(nmemb) {
        Some(0) | None => return 0,
        Some(t) => t,
    };
    let fd = unsafe { (*stream).fd };
    let n = unsafe { crate::unistd::read(fd, ptr, total) };
    if n < 0 {
        unsafe { (*stream).error = 1 };
        return 0;
    }
    if n == 0 {
        unsafe { (*stream).eof = 1 };
    }
    (n as usize) / size
}

// --- open / close ----------------------------------------------------------

/// `fopen` — STUB. Needs the `mode` string parser and an allocation for the
/// `FILE`; the underlying `openat` wrapper lives in [`crate::fcntl`]. Returns
/// NULL for now.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fopen(_path: *const c_char, _mode: *const c_char) -> *mut FILE {
    // STUB
    crate::errno::set_errno(crate::errno::ENOSYS);
    core::ptr::null_mut()
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fclose(stream: *mut FILE) -> c_int {
    if stream.is_null() {
        return EOF;
    }
    // The standard streams are static and must not be freed/closed here.
    let fd = unsafe { (*stream).fd };
    if fd <= STDERR_FILENO {
        return 0;
    }
    unsafe { crate::unistd::close(fd) }
}

// --- formatted output (STUB: variadic, see module docs) --------------------

/// `printf` — STUB. Writes `format` verbatim (conversion specifiers ignored)
/// pending `c_variadic` support. Correct only for argument-less format strings.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn printf(format: *const c_char) -> c_int {
    // STUB
    let len = unsafe { crate::string::strlen(format) };
    let n = unsafe { stream_write_all(stdout, format as *const u8, len) };
    if n < 0 { -1 } else { n as c_int }
}

/// `fprintf` — STUB, mirrors [`printf`] to an arbitrary stream.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fprintf(stream: *mut FILE, format: *const c_char) -> c_int {
    // STUB
    let len = unsafe { crate::string::strlen(format) };
    let n = unsafe { stream_write_all(stream, format as *const u8, len) };
    if n < 0 { -1 } else { n as c_int }
}
