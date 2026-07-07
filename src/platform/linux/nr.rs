//! Linux syscall numbers.
//!
//! These differ per architecture. Only the syscalls the crate currently issues
//! are listed; the table grows as modules are filled in. Values are taken from
//! the kernel `unistd.h` / syscall tables for each arch.
//!
//! aarch64 uses the "generic" (asm-generic) syscall ABI, which is also what
//! newer arches inherit.

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    pub const READ: usize = 0;
    pub const WRITE: usize = 1;
    pub const OPEN: usize = 2;
    pub const CLOSE: usize = 3;
    pub const STAT: usize = 4;
    pub const FSTAT: usize = 5;
    pub const POLL: usize = 7;
    pub const LSEEK: usize = 8;
    pub const MADVISE: usize = 28;
    pub const SCHED_YIELD: usize = 24;
    pub const FCHMOD: usize = 91;
    pub const MKDIRAT: usize = 258;
    pub const FCHMODAT: usize = 268;
    pub const NEWFSTATAT: usize = 262;
    pub const MMAP: usize = 9;
    pub const MPROTECT: usize = 10;
    pub const MUNMAP: usize = 11;
    pub const BRK: usize = 12;
    pub const RT_SIGACTION: usize = 13;
    pub const RT_SIGPROCMASK: usize = 14;
    pub const IOCTL: usize = 16;
    pub const PREAD64: usize = 17;
    pub const PWRITE64: usize = 18;
    pub const READV: usize = 19;
    pub const WRITEV: usize = 20;
    pub const NANOSLEEP: usize = 35;
    pub const GETPID: usize = 39;
    pub const CLONE: usize = 56;
    pub const FORK: usize = 57;
    pub const EXECVE: usize = 59;
    pub const EXIT: usize = 60;
    pub const WAIT4: usize = 61;
    pub const KILL: usize = 62;
    pub const UNAME: usize = 63;
    pub const FCNTL: usize = 72;
    pub const GETCWD: usize = 79;
    pub const GETDENTS64: usize = 217;
    pub const CLOCK_GETTIME: usize = 228;
    pub const CLOCK_NANOSLEEP: usize = 230;
    pub const EXIT_GROUP: usize = 231;
    pub const OPENAT: usize = 257;
}

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    // asm-generic numbering.
    pub const IOCTL: usize = 29;
    pub const MKDIRAT: usize = 34;
    pub const FCHMOD: usize = 52;
    pub const FCHMODAT: usize = 53;
    pub const NEWFSTATAT: usize = 79;
    pub const PPOLL: usize = 73;
    pub const SCHED_YIELD: usize = 124;
    pub const MADVISE: usize = 233;
    pub const OPENAT: usize = 56;
    pub const CLOSE: usize = 57;
    pub const LSEEK: usize = 62;
    pub const READ: usize = 63;
    pub const WRITE: usize = 64;
    pub const READV: usize = 65;
    pub const WRITEV: usize = 66;
    pub const PREAD64: usize = 67;
    pub const PWRITE64: usize = 68;
    pub const FSTAT: usize = 80;
    pub const EXIT: usize = 93;
    pub const EXIT_GROUP: usize = 94;
    pub const CLOCK_GETTIME: usize = 113;
    pub const CLOCK_NANOSLEEP: usize = 115;
    pub const NANOSLEEP: usize = 101;
    pub const KILL: usize = 129;
    pub const RT_SIGACTION: usize = 134;
    pub const RT_SIGPROCMASK: usize = 135;
    pub const UNAME: usize = 160;
    pub const GETPID: usize = 172;
    pub const BRK: usize = 214;
    pub const MUNMAP: usize = 215;
    pub const CLONE: usize = 220;
    pub const EXECVE: usize = 221;
    pub const MMAP: usize = 222;
    pub const MPROTECT: usize = 226;
    pub const WAIT4: usize = 260;
    pub const GETDENTS64: usize = 61;
    pub const FCNTL: usize = 25;
    pub const GETCWD: usize = 17;

    // aarch64 has no bare open/stat/fork; callers use the *at / clone variants.
    // Provide aliases so shared code compiles; they route through the modern
    // syscalls at the wrapper layer.
    pub const OPEN: usize = OPENAT;
    pub const STAT: usize = FSTAT;
    pub const FORK: usize = CLONE;
}
