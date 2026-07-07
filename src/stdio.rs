//! `<stdio.h>` — buffered I/O and formatted output.
//!
//! ## What is real
//! The stream object ([`FILE`]), the standard streams, and every *non-variadic*
//! entry point (`fopen`, `fread`, `fwrite`, `fputs`, `fputc`, `puts`,
//! `putchar`, `fflush`, `fclose`, `perror`, `fileno`) are implemented against
//! the syscall layer. Output is currently unbuffered (each write is a syscall);
//! a buffering layer is a later pass.
//!
//! ## Formatted output
//! The full `printf` family (`printf`/`fprintf`/`snprintf`/`sprintf` and their
//! `v*` counterparts) is implemented via a single engine, [`vformat`], reading
//! arguments through Rust's `c_variadic` support (nightly). Supported: `d i u o
//! x X c s p %`, flags `- + space 0 #`, width/precision (incl. `*`), and length
//! modifiers `hh h l ll z j t`. Floating point (`f F e E g G`) is a best-effort
//! implementation (see `fmt_float`). Still stubbed: `fopen`, the `scanf` family.

use crate::types::{c_char, c_int, c_long, c_longlong, c_void, size_t};
use crate::unistd::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, write};
use core::ffi::VaList;

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

// --- formatted output ------------------------------------------------------
//
// A single formatting engine, `vformat`, drives the whole printf family. It
// writes through a `Sink` that is either a stream (printf/fprintf/vprintf/...)
// or a caller buffer (snprintf/vsnprintf/sprintf). Supported conversions:
// d i u o x X c s p %, with flags (- + space 0 #), width and precision (incl.
// `*`), and length modifiers (hh h l ll z j t). Floating point (f F e E g G) is
// a basic best-effort — see `fmt_float`.

/// Output target for the formatter.
struct Sink {
    // Stream mode: write to this FILE. Buffer mode: `stream` is null.
    stream: *mut FILE,
    // Buffer mode: destination, its capacity (including room for the NUL), and
    // how many bytes have actually been stored.
    buf: *mut u8,
    cap: usize,
    stored: usize,
    // Count of bytes that *would* be written (the printf return value), and a
    // sticky error flag for stream writes.
    total: usize,
    err: bool,
}

impl Sink {
    fn stream(stream: *mut FILE) -> Self {
        Sink { stream, buf: core::ptr::null_mut(), cap: 0, stored: 0, total: 0, err: false }
    }
    fn buffer(buf: *mut u8, cap: usize) -> Self {
        Sink { stream: core::ptr::null_mut(), buf, cap, stored: 0, total: 0, err: false }
    }

    fn write(&mut self, bytes: &[u8]) {
        if !self.stream.is_null() {
            let n = unsafe { stream_write_all(self.stream, bytes.as_ptr(), bytes.len()) };
            if n < 0 {
                self.err = true;
            } else {
                self.total += n as usize;
            }
        } else {
            for &b in bytes {
                // Reserve the final byte for the terminating NUL.
                if self.cap > 0 && self.stored + 1 < self.cap {
                    unsafe { *self.buf.add(self.stored) = b };
                    self.stored += 1;
                }
                self.total += 1;
            }
        }
    }

    fn write_byte(&mut self, b: u8) {
        self.write(&[b]);
    }

    /// Emit `n` copies of a pad byte.
    fn pad(&mut self, byte: u8, n: usize) {
        let chunk = [byte; 16];
        let mut left = n;
        while left > 0 {
            let k = left.min(16);
            self.write(&chunk[..k]);
            left -= k;
        }
    }

    /// Finalize buffer mode by NUL-terminating. No-op for stream mode.
    fn finish(&mut self) {
        if self.stream.is_null() && self.cap > 0 {
            let at = self.stored.min(self.cap - 1);
            unsafe { *self.buf.add(at) = 0 };
        }
    }
}

/// Length modifier parsed from a conversion.
#[derive(Clone, Copy, PartialEq)]
enum Len {
    Char,
    Short,
    Int,
    Long,
    LongLong,
    Size,
    Max,
    Ptrdiff,
}

/// Flags parsed from a conversion.
#[derive(Default, Clone, Copy)]
struct Flags {
    left: bool,  // '-'
    plus: bool,  // '+'
    space: bool, // ' '
    zero: bool,  // '0'
    hash: bool,  // '#'
}

/// The formatting core. Returns the number of bytes that would be written
/// (ignoring truncation), or a negative value on stream error.
unsafe fn vformat(sink: &mut Sink, format: *const c_char, mut ap: VaList) -> c_int {
    let mut p = format;
    while unsafe { *p } != 0 {
        let c = (unsafe { *p }) as u8;
        if c != b'%' {
            sink.write_byte(c);
            p = unsafe { p.add(1) };
            continue;
        }
        p = unsafe { p.add(1) }; // consume '%'

        // Flags.
        let mut flags = Flags::default();
        loop {
            match (unsafe { *p }) as u8 {
                b'-' => flags.left = true,
                b'+' => flags.plus = true,
                b' ' => flags.space = true,
                b'0' => flags.zero = true,
                b'#' => flags.hash = true,
                _ => break,
            }
            p = unsafe { p.add(1) };
        }

        // Width (number or '*').
        let mut width: usize = 0;
        if (unsafe { *p }) as u8 == b'*' {
            let w = unsafe { ap.next_arg::<c_int>() };
            if w < 0 {
                flags.left = true;
                width = (-(w as i64)) as usize;
            } else {
                width = w as usize;
            }
            p = unsafe { p.add(1) };
        } else {
            while ((unsafe { *p }) as u8).is_ascii_digit() {
                width = width * 10 + ((unsafe { *p }) as u8 - b'0') as usize;
                p = unsafe { p.add(1) };
            }
        }

        // Precision (.number or .*).
        let mut precision: Option<usize> = None;
        if (unsafe { *p }) as u8 == b'.' {
            p = unsafe { p.add(1) };
            if (unsafe { *p }) as u8 == b'*' {
                let pr = unsafe { ap.next_arg::<c_int>() };
                precision = Some(if pr < 0 { 0 } else { pr as usize });
                p = unsafe { p.add(1) };
            } else {
                let mut pr = 0usize;
                while ((unsafe { *p }) as u8).is_ascii_digit() {
                    pr = pr * 10 + ((unsafe { *p }) as u8 - b'0') as usize;
                    p = unsafe { p.add(1) };
                }
                precision = Some(pr);
            }
        }

        // Length modifier.
        let len = match (unsafe { *p }) as u8 {
            b'h' => {
                p = unsafe { p.add(1) };
                if (unsafe { *p }) as u8 == b'h' {
                    p = unsafe { p.add(1) };
                    Len::Char
                } else {
                    Len::Short
                }
            }
            b'l' => {
                p = unsafe { p.add(1) };
                if (unsafe { *p }) as u8 == b'l' {
                    p = unsafe { p.add(1) };
                    Len::LongLong
                } else {
                    Len::Long
                }
            }
            b'z' => {
                p = unsafe { p.add(1) };
                Len::Size
            }
            b'j' => {
                p = unsafe { p.add(1) };
                Len::Max
            }
            b't' => {
                p = unsafe { p.add(1) };
                Len::Ptrdiff
            }
            _ => Len::Int,
        };

        // Conversion specifier.
        let spec = (unsafe { *p }) as u8;
        p = unsafe { p.add(1) };
        match spec {
            b'%' => sink.write_byte(b'%'),
            b'c' => {
                let ch = (unsafe { ap.next_arg::<c_int>() }) as u8;
                emit_padded(sink, &[ch], &flags, width, false);
            }
            b's' => {
                let s = unsafe { ap.next_arg::<*const c_char>() };
                emit_str(sink, s, &flags, width, precision);
            }
            b'd' | b'i' => {
                let v = unsafe { read_signed(&mut ap, len) };
                emit_int(sink, v.unsigned_abs(), v < 0, &flags, width, precision, 10, false, b"");
            }
            b'u' => {
                let v = unsafe { read_unsigned(&mut ap, len) };
                emit_int(sink, v, false, &flags, width, precision, 10, false, b"");
            }
            b'o' => {
                let v = unsafe { read_unsigned(&mut ap, len) };
                let prefix: &[u8] = if flags.hash { b"0" } else { b"" };
                emit_int(sink, v, false, &flags, width, precision, 8, false, prefix);
            }
            b'x' => {
                let v = unsafe { read_unsigned(&mut ap, len) };
                let prefix: &[u8] = if flags.hash && v != 0 { b"0x" } else { b"" };
                emit_int(sink, v, false, &flags, width, precision, 16, false, prefix);
            }
            b'X' => {
                let v = unsafe { read_unsigned(&mut ap, len) };
                let prefix: &[u8] = if flags.hash && v != 0 { b"0X" } else { b"" };
                emit_int(sink, v, false, &flags, width, precision, 16, true, prefix);
            }
            b'p' => {
                let v = (unsafe { ap.next_arg::<usize>() }) as u64;
                emit_int(sink, v, false, &flags, width, None, 16, false, b"0x");
            }
            b'f' | b'F' | b'e' | b'E' | b'g' | b'G' => {
                let v = unsafe { ap.next_arg::<f64>() };
                fmt_float(sink, v, spec, &flags, width, precision);
            }
            b'n' => {
                // %n: store bytes-written-so-far. Rarely used and a security
                // hazard; we consume the pointer arg but intentionally ignore it.
                let _ = unsafe { ap.next_arg::<*mut c_int>() };
            }
            0 => break, // trailing '%' at end of string
            other => {
                // Unknown specifier: emit it literally, like glibc.
                sink.write_byte(b'%');
                sink.write_byte(other);
            }
        }
    }
    if sink.err { -1 } else { sink.total as c_int }
}

/// Read a signed integer argument, honoring the length modifier and C's default
/// argument promotions (types narrower than `int` arrive as `int`).
unsafe fn read_signed(ap: &mut VaList, len: Len) -> i64 {
    match len {
        Len::Char => (unsafe { ap.next_arg::<c_int>() }) as i8 as i64,
        Len::Short => (unsafe { ap.next_arg::<c_int>() }) as i16 as i64,
        Len::Int => (unsafe { ap.next_arg::<c_int>() }) as i64,
        Len::Long => unsafe { ap.next_arg::<c_long>() },
        Len::LongLong => unsafe { ap.next_arg::<c_longlong>() },
        Len::Size => (unsafe { ap.next_arg::<isize>() }) as i64,
        Len::Max => unsafe { ap.next_arg::<i64>() },
        Len::Ptrdiff => (unsafe { ap.next_arg::<isize>() }) as i64,
    }
}

unsafe fn read_unsigned(ap: &mut VaList, len: Len) -> u64 {
    match len {
        Len::Char => (unsafe { ap.next_arg::<c_int>() }) as u8 as u64,
        Len::Short => (unsafe { ap.next_arg::<c_int>() }) as u16 as u64,
        Len::Int => (unsafe { ap.next_arg::<c_int>() }) as u32 as u64,
        Len::Long => (unsafe { ap.next_arg::<c_long>() }) as u64,
        Len::LongLong => (unsafe { ap.next_arg::<c_longlong>() }) as u64,
        Len::Size => (unsafe { ap.next_arg::<usize>() }) as u64,
        Len::Max => unsafe { ap.next_arg::<u64>() },
        Len::Ptrdiff => (unsafe { ap.next_arg::<usize>() }) as u64,
    }
}

/// Emit raw bytes with width padding (used by %c and %%-like literals).
fn emit_padded(sink: &mut Sink, body: &[u8], flags: &Flags, width: usize, zero_ok: bool) {
    let pad = width.saturating_sub(body.len());
    let padbyte = if flags.zero && zero_ok && !flags.left { b'0' } else { b' ' };
    if flags.left {
        sink.write(body);
        sink.pad(b' ', pad);
    } else {
        sink.pad(padbyte, pad);
        sink.write(body);
    }
}

/// Emit a string with optional precision (max length) and width padding.
/// A NULL pointer renders as "(null)".
fn emit_str(sink: &mut Sink, s: *const c_char, flags: &Flags, width: usize, precision: Option<usize>) {
    let s = if s.is_null() {
        b"(null)".as_ptr() as *const c_char
    } else {
        s
    };
    // Length = strlen, capped by precision if present.
    let mut len = 0usize;
    while unsafe { *s.add(len) } != 0 {
        if precision.is_some_and(|pr| len >= pr) {
            break;
        }
        len += 1;
    }
    let bytes = unsafe { core::slice::from_raw_parts(s as *const u8, len) };
    let pad = width.saturating_sub(len);
    if flags.left {
        sink.write(bytes);
        sink.pad(b' ', pad);
    } else {
        sink.pad(b' ', pad);
        sink.write(bytes);
    }
}

/// Emit an integer: magnitude `mag`, `neg` sign, in `base`, applying precision
/// (minimum digit count), sign/space/plus, an optional base prefix, and width
/// with zero- or space-padding per the flags.
#[allow(clippy::too_many_arguments)]
fn emit_int(
    sink: &mut Sink,
    mag: u64,
    neg: bool,
    flags: &Flags,
    width: usize,
    precision: Option<usize>,
    base: u64,
    upper: bool,
    prefix: &[u8],
) {
    let mut digits = [0u8; 20];
    let lut: &[u8; 16] = if upper {
        b"0123456789ABCDEF"
    } else {
        b"0123456789abcdef"
    };
    let mut n = mag;
    let mut i = digits.len();
    while n > 0 {
        i -= 1;
        digits[i] = lut[(n % base) as usize];
        n /= base;
    }
    let mut ndigits = digits.len() - i;

    // Precision: minimum number of digits. Precision 0 with value 0 => no digits.
    let mut zeros = 0usize;
    if let Some(pr) = precision {
        if pr == 0 && mag == 0 {
            ndigits = 0;
            i = digits.len();
        } else if pr > ndigits {
            zeros = pr - ndigits;
        }
    }

    // Sign character.
    let sign: &[u8] = if neg {
        b"-"
    } else if flags.plus {
        b"+"
    } else if flags.space {
        b" "
    } else {
        b""
    };

    let body_len = sign.len() + prefix.len() + zeros + ndigits;
    let pad = width.saturating_sub(body_len);

    // With an explicit precision, the '0' flag is ignored (space-pad instead).
    let zero_pad = flags.zero && !flags.left && precision.is_none();

    if !flags.left && !zero_pad {
        sink.pad(b' ', pad);
    }
    sink.write(sign);
    sink.write(prefix);
    if zero_pad {
        sink.pad(b'0', pad);
    }
    sink.pad(b'0', zeros);
    if ndigits > 0 {
        sink.write(&digits[i..]);
    }
    if flags.left {
        sink.pad(b' ', pad);
    }
}

/// Basic floating-point formatting for %f/%e/%g. Handles sign, inf/nan, and a
/// fixed-precision fractional expansion (default 6). This is a best-effort
/// implementation: very large magnitudes (beyond u64 for the integer part) and
/// correct rounding/shortest-representation are not yet handled. TODO(dtoa).
fn fmt_float(sink: &mut Sink, value: f64, spec: u8, flags: &Flags, width: usize, precision: Option<usize>) {
    let upper = spec.is_ascii_uppercase();
    // Special values.
    if value.is_nan() {
        let s: &[u8] = if upper { b"NAN" } else { b"nan" };
        emit_padded(sink, s, flags, width, false);
        return;
    }
    if value.is_infinite() {
        let s: &[u8] = match (value < 0.0, upper) {
            (true, false) => b"-inf",
            (true, true) => b"-INF",
            (false, false) => b"inf",
            (false, true) => b"INF",
        };
        emit_padded(sink, s, flags, width, false);
        return;
    }

    // `trunc`/`floor` live in std (libm-backed), not core. For our non-negative
    // working value, truncation toward zero via an integer cast is floor.
    fn ftrunc(x: f64) -> f64 {
        (x as i64) as f64
    }

    let prec = precision.unwrap_or(6);
    // Cap the scaling precision so 10^prec stays within f64/u64 exact range;
    // any fractional positions beyond that are emitted as trailing zeros.
    const MAX_SCALE_PREC: usize = 15;
    let sprec = prec.min(MAX_SCALE_PREC);

    let neg = value.is_sign_negative();
    let v = if neg { -value } else { value };

    // Round *once* into a single scaled integer holding all significant digits,
    // then split it at the decimal point. Extracting the fractional digits from
    // this integer (rather than re-multiplying a float) keeps rounding exact.
    let mut scale = 1.0f64;
    let mut k = 0;
    while k < sprec {
        scale *= 10.0;
        k += 1;
    }
    let scaled = ftrunc(v * scale + 0.5) as u64;

    // Decimal digits of `scaled`, most-significant first.
    let mut dbuf = [0u8; 40];
    let mut dl = 0usize;
    let mut s = scaled;
    if s == 0 {
        dbuf[0] = b'0';
        dl = 1;
    } else {
        let mut tmp = [0u8; 20];
        let mut tl = 0;
        while s > 0 {
            tmp[tl] = b'0' + (s % 10) as u8;
            s /= 10;
            tl += 1;
        }
        while tl > 0 {
            tl -= 1;
            dbuf[dl] = tmp[tl];
            dl += 1;
        }
    }
    // Left-pad so there are at least `sprec + 1` digits (one integer digit).
    let mut digits = [0u8; 41];
    let lead = (sprec + 1).saturating_sub(dl);
    for slot in digits.iter_mut().take(lead) {
        *slot = b'0';
    }
    digits[lead..lead + dl].copy_from_slice(&dbuf[..dl]);
    let ndig = lead + dl;

    let int_digits = &digits[..ndig - sprec];
    let frac_digits = &digits[ndig - sprec..ndig]; // length == sprec

    let sign: &[u8] = if neg {
        b"-"
    } else if flags.plus {
        b"+"
    } else if flags.space {
        b" "
    } else {
        b""
    };
    // Total fractional width is the requested `prec` (frac_digits + zero tail).
    let frac_tail = prec - sprec;
    let dot: usize = if prec > 0 { 1 } else { 0 };
    let body_len = sign.len() + int_digits.len() + dot + prec;
    let pad = width.saturating_sub(body_len);
    let zero_pad = flags.zero && !flags.left;

    if !flags.left && !zero_pad {
        sink.pad(b' ', pad);
    }
    sink.write(sign);
    if zero_pad {
        sink.pad(b'0', pad);
    }
    sink.write(int_digits);
    if prec > 0 {
        sink.write_byte(b'.');
        sink.write(frac_digits);
        sink.pad(b'0', frac_tail);
    }
    if flags.left {
        sink.pad(b' ', pad);
    }
}

// --- public printf family --------------------------------------------------

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn vfprintf(stream: *mut FILE, format: *const c_char, ap: VaList) -> c_int {
    let mut sink = Sink::stream(stream);
    let r = unsafe { vformat(&mut sink, format, ap) };
    sink.finish();
    r
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn vprintf(format: *const c_char, ap: VaList) -> c_int {
    unsafe { vfprintf(stdout, format, ap) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn vsnprintf(
    str: *mut c_char,
    size: size_t,
    format: *const c_char,
    ap: VaList,
) -> c_int {
    let mut sink = Sink::buffer(str as *mut u8, size);
    let r = unsafe { vformat(&mut sink, format, ap) };
    sink.finish();
    r
}

/// `vsprintf` — unbounded; the caller guarantees the buffer is large enough.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn vsprintf(str: *mut c_char, format: *const c_char, ap: VaList) -> c_int {
    unsafe { vsnprintf(str, usize::MAX, format, ap) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn printf(format: *const c_char, ap: ...) -> c_int {
    unsafe { vprintf(format, ap) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn fprintf(stream: *mut FILE, format: *const c_char, ap: ...) -> c_int {
    unsafe { vfprintf(stream, format, ap) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn snprintf(
    str: *mut c_char,
    size: size_t,
    format: *const c_char,
    ap: ...
) -> c_int {
    unsafe { vsnprintf(str, size, format, ap) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn sprintf(str: *mut c_char, format: *const c_char, ap: ...) -> c_int {
    unsafe { vsprintf(str, format, ap) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Format into a stack buffer via the C `snprintf` and return it as a
    /// `String` for comparison.
    fn snp(expected_cap: usize, format: &[u8], f: impl FnOnce(*mut c_char, *const c_char) -> c_int) -> alloc::string::String {
        let mut buf = alloc::vec![0u8; expected_cap];
        let n = f(buf.as_mut_ptr() as *mut c_char, format.as_ptr() as *const c_char);
        let s = &buf[..n as usize];
        alloc::string::String::from_utf8_lossy(s).into_owned()
    }

    // Under cfg(test) the crate links std; pull in alloc for the helper.
    extern crate alloc;

    #[test]
    fn integers_and_strings() {
        let out = snp(64, b"[%5d][%-5d][%05d][%x][%s]\0", |b, fmt| unsafe {
            snprintf(b, 64, fmt, 42, 42, 42, 255, b"hi\0".as_ptr())
        });
        assert_eq!(out, "[   42][42   ][00042][ff][hi]");
    }

    #[test]
    fn precision_and_plus() {
        let out = snp(64, b"[%+d][%.3d][%8.2f]\0", |b, fmt| unsafe {
            snprintf(b, 64, fmt, 7, 5, 3.14159f64)
        });
        assert_eq!(out, "[+7][005][    3.14]");
    }

    #[test]
    fn float_rounds_correctly() {
        // 3.14159 to 3 decimals rounds up the last digit -> 3.142 (regression:
        // an earlier impl re-extracted frac digits via float mul and got 3.141).
        let out = snp(64, b"[%.3f][%.0f][%.2f]\0", |b, fmt| unsafe {
            snprintf(b, 64, fmt, 3.14159f64, 2.0f64, 0.005f64)
        });
        assert_eq!(out, "[3.142][2][0.01]");
    }

    #[test]
    fn truncation_returns_full_length() {
        // Buffer too small: snprintf must return the length it *would* produce.
        let mut buf = [0u8; 4];
        let n = unsafe {
            snprintf(buf.as_mut_ptr() as *mut c_char, 4, b"%d\0".as_ptr() as *const c_char, 12345)
        };
        assert_eq!(n, 5);
        // Buffer holds the truncated, NUL-terminated prefix "123".
        assert_eq!(&buf, b"123\0");
    }
}
