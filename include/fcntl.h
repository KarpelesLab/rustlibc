/* fcntl.h — file control (POSIX).
 *
 * Part of rustlibc. open/openat/fcntl are implemented. Note: open() here is a
 * fixed-arity wrapper (mode = 0) until variadic support lands; use openat() to
 * create files with an explicit mode. See src/fcntl.rs. */
#ifndef _RUSTLIBC_FCNTL_H
#define _RUSTLIBC_FCNTL_H

#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

#define O_RDONLY 00
#define O_WRONLY 01
#define O_RDWR 02
#define O_CREAT 0100
#define O_EXCL 0200
#define O_NOCTTY 0400
#define O_TRUNC 01000
#define O_APPEND 02000
#define O_NONBLOCK 04000
#define O_DIRECTORY 0200000
#define O_CLOEXEC 02000000

#define AT_FDCWD (-100)

int open(const char *pathname, int flags);
int openat(int dirfd, const char *pathname, int flags, mode_t mode);
int fcntl(int fd, int cmd, int arg);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_FCNTL_H */
