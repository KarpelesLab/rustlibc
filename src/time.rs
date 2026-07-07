//! `<time.h>` — clocks and (eventually) calendar time.
//!
//! `clock_gettime` and `time` are real. Calendar conversions (`gmtime`,
//! `localtime`, `strftime`, `mktime`) are **stubbed** — they need the civil-time
//! algorithms and timezone handling, which is a focused later effort.

use crate::platform::linux::{nr, syscall2};
use crate::types::{c_int, clockid_t, time_t, timespec};

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;

/// Broken-down calendar time (`struct tm`). Layout matches glibc/musl.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: crate::types::c_long,
    pub tm_zone: *const crate::types::c_char,
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int {
    unsafe { syscall2(nr::CLOCK_GETTIME, clk_id as usize, tp as usize) as c_int }
}

/// `time` — seconds since the epoch. Implemented on top of `clock_gettime`.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn time(tloc: *mut time_t) -> time_t {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if unsafe { clock_gettime(CLOCK_REALTIME, &mut ts) } != 0 {
        return -1;
    }
    if !tloc.is_null() {
        unsafe { *tloc = ts.tv_sec };
    }
    ts.tv_sec
}

/// `gmtime_r` — STUB. Civil-time conversion not yet implemented.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn gmtime_r(_timep: *const time_t, result: *mut tm) -> *mut tm {
    // STUB
    crate::errno::set_errno(crate::errno::ENOSYS);
    if result.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { *result = tm::default() };
        result
    }
}
