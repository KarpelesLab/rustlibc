//! `<sys/utsname.h>` — system identification.
//!
//! Real wrapper over `uname`. The field length (65) matches the Linux
//! `new_utsname` struct (`__NEW_UTS_LEN + 1`).

use crate::platform::linux::{nr, syscall1};
use crate::types::{c_char, c_int};

const UTS_LEN: usize = 65;

/// `struct utsname` — kernel `new_utsname` layout.
#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; UTS_LEN],
    pub nodename: [c_char; UTS_LEN],
    pub release: [c_char; UTS_LEN],
    pub version: [c_char; UTS_LEN],
    pub machine: [c_char; UTS_LEN],
    /// GNU extension; present in the kernel struct as `domainname`.
    pub domainname: [c_char; UTS_LEN],
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn uname(buf: *mut utsname) -> c_int {
    unsafe { syscall1(nr::UNAME, buf as usize) as c_int }
}
