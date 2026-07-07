# Build helpers for rustlibc.
#
#   make lib        build the static + shared libs (release) for $(ARCH)
#   make hello      build examples/hello.c linked against the static lib
#   make run        build and run the hello example
#   make test       run the Rust unit tests
#   make clean      remove build artifacts
#
# Cross-compile by overriding ARCH, e.g.:
#   make hello ARCH=aarch64-unknown-linux-gnu

ARCH ?= x86_64-unknown-linux-gnu
PROFILE ?= release
LIBDIR := target/$(ARCH)/$(PROFILE)
STATICLIB := $(LIBDIR)/librustlibc.a

# Freestanding link: no system headers, no system libc, no system crt — rustlibc
# provides the headers (-Iinclude), the C functions, and _start.
CFLAGS := -nostdinc -Iinclude -Wall -O2
LDFLAGS := -nostdlib -nostartfiles -no-pie -static

.PHONY: all lib hello run test clean

all: lib

lib:
	cargo build --$(PROFILE) --target $(ARCH)

$(STATICLIB): lib

hello: examples/hello.c $(STATICLIB)
	cc $(CFLAGS) $(LDFLAGS) $< $(STATICLIB) -o hello

run: hello
	./hello

test:
	cargo test --lib

clean:
	cargo clean
	rm -f hello
