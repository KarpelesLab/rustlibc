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

Toolchain is pinned to **Rust 1.88** (edition 2024) via `rust-toolchain.toml`.

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
include/           the C headers programs #include (self-contained)
examples/hello.c   a freestanding C program linked against rustlibc
```

Every user-visible libc function is exported with the C ABI under its canonical
name via `#[cfg_attr(not(test), unsafe(no_mangle))] extern "C"`. The `not(test)`
gate drops the C symbols during `cargo test` (where the std harness already
links the system libc); unit tests call the Rust paths directly.

## Building & running

```sh
make lib                       # build static + shared libs (release)
make run                       # build & run examples/hello.c against the .a
make test                      # cargo test --lib
make hello ARCH=aarch64-unknown-linux-gnu   # cross-build the example
```

`make run` links `hello.c` fully freestanding
(`-nostdinc -Iinclude -nostdlib -nostartfiles -static`) and prints:

```
Hello from rustlibc!
argv[0] = ./hello
===============================
1234
```

## Status

**Implemented (real):**
- `string.h` / `strings.h`: mem*, str* family, `strerror`, `bcmp`.
- `ctype.h`: full classification/conversion (C locale).
- `stdlib.h`: `strtol`/`atoi` family, `abs`, `qsort`, `bsearch`, `exit`,
  `abort`, `__assert_fail`.
- `malloc.h`/`stdlib.h`: `malloc`/`free`/`calloc`/`realloc`/`aligned_alloc`
  (one `mmap` per allocation — correct, not yet optimized).
- `unistd.h`: `read`/`write`/`close`/`lseek`/`getpid`/`_exit`/`sleep`/`getcwd`.
- `fcntl.h`: `open`/`openat`/`fcntl`.
- `stdio.h`: `FILE`, standard streams, `fwrite`/`fread`/`fputs`/`fputc`/`puts`/
  `putchar`/`perror`/`fileno`/`fflush` (unbuffered).
- `time.h`: `clock_gettime`, `time`.
- `signal.h`: numbers, `kill`, `raise`.
- `math.h`: `fabs`/`copysign`/`fmax`/`fmin`/`fmod`/`sqrt`.
- `setjmp.h`: `setjmp`/`longjmp` (naked asm, both arches).
- `crt`: `_start` → `main`, `environ` capture.

**Stubbed (marked `// STUB`, wired to link but not functional):**
- **`printf`/`fprintf`/`scanf` family** — see the limitation below.
- `fopen`, `getenv`, `signal` (handler install), `gmtime_r`, `mbrtowc`,
  math transcendentals (`sin`/`cos`/`exp`/`log`/`pow`/…).

### Known limitation: variadic functions

The pinned stable Rust 1.88 toolchain does **not** provide the unstable
`c_variadic` feature, which is required to *consume* a C `va_list` in Rust.
Consequently `printf`/`scanf`-style functions cannot yet read their variadic
arguments. The stubs emit the format string verbatim (correct only for
argument-less calls) so C programs still link. Resolving this needs one of:
a toolchain that enables `c_variadic`, or per-arch assembly `va_list` shims.
This is tracked as the top open design question.

## Roadmap

1. Variadic story → real `printf`/`snprintf`/`scanf`.
2. Buffered stdio; `fopen`/`fdopen`.
3. A real segregated-free-list allocator.
4. Threads (`pthread`), TLS, per-thread `errno`.
5. Signal delivery (trampoline + `sigaction`).
6. `libm` port for the math transcendentals.
7. Dynamic-linker / full static-PIE story.

## License

MIT — see [LICENSE](LICENSE).
