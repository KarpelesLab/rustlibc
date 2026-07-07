# rustlibc

A full [libc](https://en.wikipedia.org/wiki/C_standard_library) implementation
as a single Rust crate, exposing C bindings (static and dynamic) for use by
ordinary C/C++ programs in place of glibc or musl.

The crate is **freestanding** (`#![no_std]`): it talks to the kernel directly
through raw syscalls and provides its own C runtime (`_start`). The resulting
`librustlibc.a` / `librustlibc.so` need no other libc underneath.

> Status: **early scaffold.** The structure spans the C standard / POSIX headers
> broadly; pure-computation surfaces are implemented and tested, while I/O,
> allocation and math are a mix of real and stubbed. See [Status](#status).

## Targets

| Target                        | Build | Notes                                   |
| ----------------------------- | :---: | --------------------------------------- |
| `x86_64-unknown-linux-gnu`    |   ✅  | Primary; end-to-end example runs.       |
| `aarch64-unknown-linux-gnu`   |   ✅  | Builds; cross-links via bundled `rust-lld`. |

Toolchain is pinned to **nightly** (edition 2024) via `rust-toolchain.toml` —
the `c_variadic` feature is required to implement the `printf` family.

## Layout

```
src/
  arch/            per-architecture primitives (raw syscall, setjmp, _start asm)
    x86_64.rs
    aarch64.rs
  platform/linux/  syscall numbers + the errno-aware syscall wrapper
  crt.rs           _start / __libc_start_main (C runtime bootstrap)
  errno.rs types.rs
  ctype.rs string.rs strings.rs stdlib.rs malloc.rs stdio.rs
  unistd.rs fcntl.rs time.rs signal.rs math.rs wchar.rs setjmp.rs
  poll.rs sched.rs dirent.rs pthread.rs locale.rs
  sys/             mman.rs stat.rs uio.rs utsname.rs wait.rs
include/           the C headers programs #include (self-contained)
examples/          hello.c (strings/printf/malloc), sysprobe.c (uname/stat/dir)
```

Every user-visible libc function is exported with the C ABI under its canonical
name via `#[cfg_attr(not(test), unsafe(no_mangle))] extern "C"`. The `not(test)`
gate drops the C symbols during `cargo test` (where the std harness already
links the system libc); unit tests call the Rust paths directly.

## Building & running

```sh
make lib                       # build static + shared libs (release)
make run                       # build & run the examples against the .a
make test                      # cargo test --lib
make hello ARCH=aarch64-unknown-linux-gnu   # cross-build an example
```

`make run` links the examples fully freestanding
(`-nostdinc -Iinclude -nostdlib -nostartfiles -static`) and prints, e.g.:

```
=== hello ===
Hello from rustlibc!
argc=1, argv[0]=./hello
int=42 hex=0xff oct=100 char=!
float=3.142 str=trun pct=%
=== sysprobe ===
uname: Linux 6.12.41-gentoo-x86_64 (x86_64)
/etc/hostname: 7 bytes, mode 644
entries in ".": ./ ../ .git/ LICENSE src/ ...
```

## Status

**Implemented (real):**
- `string.h` / `strings.h`: mem*, str* family, `strerror`, `bcmp`.
- `ctype.h`: full classification/conversion (C locale).
- `stdio.h`: `FILE`, standard streams, `fwrite`/`fread`/`fputs`/`fputc`/`puts`/
  `putchar`/`perror`/`fileno`/`fflush` (unbuffered), and the full **`printf`
  family** — `printf`/`fprintf`/`snprintf`/`sprintf` + `v*` — via `c_variadic`
  (integers/strings/char/pointer complete; floats best-effort).
- `stdlib.h`: `strtol`/`atoi` family, `abs`/`labs`/`llabs`/`imaxabs`, `qsort`,
  `bsearch`, `exit`, `abort`, `__assert_fail`.
- `malloc.h`/`stdlib.h`: `malloc`/`free`/`calloc`/`realloc`/`aligned_alloc`
  (one `mmap` per allocation — correct, not yet optimized).
- `unistd.h`: `read`/`write`/`close`/`lseek`/`getpid`/`_exit`/`sleep`/`getcwd`.
- `fcntl.h`: `open`/`openat`/`fcntl`.
- `sys/stat.h`: per-arch `struct stat`, `stat`/`lstat`/`fstat`/`fstatat`,
  `mkdir`/`chmod`/`fchmod`.
- `sys/mman.h`: `mmap`/`munmap`/`mprotect`/`madvise`.
- `sys/uio.h` `readv`/`writev`; `sys/utsname.h` `uname`; `sys/wait.h`
  `wait`/`waitpid` (+ `W*` macros); `poll.h` `poll`; `sched.h` `sched_yield`.
- `dirent.h`: `opendir`/`readdir`/`closedir`/`dirfd` (over `getdents64`).
- `time.h`: `clock_gettime`, `time`. `signal.h`: numbers, `kill`, `raise`.
- `math.h`: `fabs`/`copysign`/`fmax`/`fmin`/`fmod`/`sqrt`.
- `setjmp.h`: `setjmp`/`longjmp` (naked asm, both arches).
- `pthread.h`: mutex/once as single-threaded no-ops (`pthread_create` stubbed).
- `crt`: `_start` → `main`, `environ` capture.
- Header-only: `stdint`/`stddef`/`stdbool`/`stdarg`/`limits`/`float`/
  `inttypes`/`iso646`/`stdalign`/`stdnoreturn`.

**Stubbed (marked `// STUB`, wired to link but not functional):**
- `scanf` family, `fopen`, `getenv`, `signal` (handler install), `pthread_create`,
  `gmtime_r`, `mbrtowc`, `setlocale` (nominal "C"),
  math transcendentals (`sin`/`cos`/`exp`/`log`/`pow`/…).

## Roadmap

1. Buffered stdio; `fopen`/`fdopen`; the `scanf` family.
2. A real segregated-free-list allocator.
3. Threads (`pthread_create`), TLS, per-thread `errno`.
4. Signal delivery (trampoline + `sigaction`).
5. `libm` port for the math transcendentals.
6. Sockets (`sys/socket.h`, `netinet/`, `arpa/`) and more `sys/*`.
7. Dynamic-linker / full static-PIE story.

## License

MIT — see [LICENSE](LICENSE).
