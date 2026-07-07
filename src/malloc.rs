//! `<stdlib.h>` allocation: `malloc` / `free` / `calloc` / `realloc`.
//!
//! This is a **correct but deliberately simple** allocator: every allocation is
//! its own anonymous `mmap` region, prefixed by a small header recording the
//! mapping length. `free` unmaps it. That wastes up to a page per object and
//! makes small allocations expensive, but it is real, thread-safe (the kernel
//! serializes `mmap`/`munmap`), and needs no global bookkeeping.
//!
//! TODO(perf): replace with a proper size-class / bins allocator (segregated
//! free lists + a page cache) so small objects don't each burn a page.

use crate::types::{c_void, size_t};

// mmap / munmap constants (Linux, arch-independent for our two targets).
const PROT_READ: usize = 0x1;
const PROT_WRITE: usize = 0x2;
const MAP_PRIVATE: usize = 0x02;
const MAP_ANONYMOUS: usize = 0x20;
const PAGE_SIZE: usize = 4096;

/// Bytes reserved before the user pointer. Holds the mapping length and keeps
/// the returned pointer 16-byte aligned (`max_align_t`), since mmap returns
/// page-aligned memory.
const HEADER: usize = 16;

#[inline]
fn round_up(n: usize, to: usize) -> usize {
    (n + to - 1) & !(to - 1)
}

#[inline]
unsafe fn sys_mmap(len: usize) -> *mut u8 {
    let ret = unsafe {
        crate::platform::linux::syscall6(
            crate::platform::linux::nr::MMAP,
            0, // addr = NULL, let the kernel choose
            len,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            usize::MAX, // fd = -1
            0,          // offset
        )
    };
    if ret < 0 {
        core::ptr::null_mut()
    } else {
        ret as usize as *mut u8
    }
}

#[inline]
unsafe fn sys_munmap(addr: *mut u8, len: usize) {
    unsafe {
        crate::platform::linux::syscall2(
            crate::platform::linux::nr::MUNMAP,
            addr as usize,
            len,
        );
    }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn malloc(size: size_t) -> *mut c_void {
    if size == 0 {
        // A unique, freeable pointer is a valid response for size 0.
        return unsafe { malloc(1) };
    }
    let total = round_up(size + HEADER, PAGE_SIZE);
    let base = unsafe { sys_mmap(total) };
    if base.is_null() {
        crate::errno::set_errno(crate::errno::ENOMEM);
        return core::ptr::null_mut();
    }
    // Store the mapping length in the header so free/realloc can recover it.
    unsafe { *(base as *mut usize) = total };
    unsafe { base.add(HEADER) as *mut c_void }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let base = unsafe { (ptr as *mut u8).sub(HEADER) };
    let total = unsafe { *(base as *const usize) };
    unsafe { sys_munmap(base, total) };
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn calloc(nmemb: size_t, size: size_t) -> *mut c_void {
    // Guard against multiplication overflow (CWE-190).
    let total = match nmemb.checked_mul(size) {
        Some(t) => t,
        None => {
            crate::errno::set_errno(crate::errno::ENOMEM);
            return core::ptr::null_mut();
        }
    };
    // Anonymous mmap memory is already zero-filled, so no explicit memset.
    unsafe { malloc(total) }
}

#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    if ptr.is_null() {
        return unsafe { malloc(size) };
    }
    if size == 0 {
        unsafe { free(ptr) };
        return core::ptr::null_mut();
    }
    let base = unsafe { (ptr as *mut u8).sub(HEADER) };
    let old_total = unsafe { *(base as *const usize) };
    let old_usable = old_total - HEADER;
    // Fast path: still fits in the existing mapping.
    if size <= old_usable {
        return ptr;
    }
    let new = unsafe { malloc(size) };
    if new.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        crate::string::memcpy(new, ptr, old_usable);
        free(ptr);
    }
    new
}

/// `aligned_alloc` (C11) — every mapping is already page-aligned, so any
/// requested alignment up to a page is trivially satisfied. Larger alignments
/// are not yet supported.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void {
    if alignment > PAGE_SIZE || !alignment.is_power_of_two() {
        crate::errno::set_errno(crate::errno::EINVAL);
        return core::ptr::null_mut();
    }
    unsafe { malloc(size) }
}
