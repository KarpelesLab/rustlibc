//! `<setjmp.h>` — non-local jumps.
//!
//! `setjmp`/`longjmp` are implemented as naked functions (stabilized in Rust
//! 1.88) that save and restore the callee-saved register set plus the stack
//! pointer and return address. Because we own both sides, we use our own
//! `jmp_buf` layout rather than matching glibc's.
//!
//! Not saved yet: the signal mask (so these are really `_setjmp`/`sigsetjmp`
//! with `savemask == 0`) and callee-saved FP registers (`d8`–`d15` on aarch64).
//! Both are TODOs before this is a fully conformant `setjmp`.

use crate::types::c_int;

/// Storage for a jump target. Sized generously (256 bytes) to leave room for
/// the future signal-mask and FP-register additions without an ABI change.
#[repr(C)]
pub struct __jmp_buf_tag {
    __data: [u64; 32],
}

/// C `jmp_buf` is an array type, so it decays to `*mut __jmp_buf_tag` at call
/// sites — which is exactly the pointer our asm expects.
pub type jmp_buf = [__jmp_buf_tag; 1];

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn setjmp(_env: *mut __jmp_buf_tag) -> c_int {
    core::arch::naked_asm!(
        "mov [rdi + 0], rbx",
        "mov [rdi + 8], rbp",
        "mov [rdi + 16], r12",
        "mov [rdi + 24], r13",
        "mov [rdi + 32], r14",
        "mov [rdi + 40], r15",
        "lea rax, [rsp + 8]", // caller's rsp (undo the return-address push)
        "mov [rdi + 48], rax",
        "mov rax, [rsp]", // return address
        "mov [rdi + 56], rax",
        "xor eax, eax", // setjmp returns 0 on the direct call
        "ret",
    )
}

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn longjmp(_env: *mut __jmp_buf_tag, _val: c_int) -> ! {
    core::arch::naked_asm!(
        "mov rbx, [rdi + 0]",
        "mov rbp, [rdi + 8]",
        "mov r12, [rdi + 16]",
        "mov r13, [rdi + 24]",
        "mov r14, [rdi + 32]",
        "mov r15, [rdi + 40]",
        "mov rsp, [rdi + 48]",
        "mov rcx, [rdi + 56]", // saved return address
        "mov eax, esi",        // proposed return value
        "test eax, eax",
        "jnz 2f",
        "inc eax", // longjmp(.., 0) must appear to setjmp as 1
        "2:",
        "jmp rcx",
    )
}

#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn setjmp(_env: *mut __jmp_buf_tag) -> c_int {
    core::arch::naked_asm!(
        "stp x19, x20, [x0, #0]",
        "stp x21, x22, [x0, #16]",
        "stp x23, x24, [x0, #32]",
        "stp x25, x26, [x0, #48]",
        "stp x27, x28, [x0, #64]",
        "stp x29, x30, [x0, #80]", // frame pointer + link register
        "mov x1, sp",
        "str x1, [x0, #96]",
        "mov x0, #0",
        "ret",
    )
}

#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C" fn longjmp(_env: *mut __jmp_buf_tag, _val: c_int) -> ! {
    core::arch::naked_asm!(
        "ldp x19, x20, [x0, #0]",
        "ldp x21, x22, [x0, #16]",
        "ldp x23, x24, [x0, #32]",
        "ldp x25, x26, [x0, #48]",
        "ldp x27, x28, [x0, #64]",
        "ldp x29, x30, [x0, #80]",
        "ldr x2, [x0, #96]",
        "mov sp, x2",
        "cmp x1, #0",
        "csinc x0, x1, xzr, ne", // x0 = (val != 0) ? val : 1
        "ret",                    // returns to restored x30 (setjmp's caller)
    )
}
