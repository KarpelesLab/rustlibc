//! `<sched.h>` — scheduling.
//!
//! `sched_yield` is real; the affinity/policy surface is a later addition.

use crate::platform::linux::{nr, syscall0};
use crate::types::c_int;

#[cfg_attr(not(test), unsafe(no_mangle))]
pub extern "C" fn sched_yield() -> c_int {
    unsafe { syscall0(nr::SCHED_YIELD) as c_int }
}
