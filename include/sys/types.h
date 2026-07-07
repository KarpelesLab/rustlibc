/* sys/types.h — POSIX system data types.
 *
 * Part of rustlibc. Widths match the Linux LP64 ABI (x86_64, aarch64). */
#ifndef _RUSTLIBC_SYS_TYPES_H
#define _RUSTLIBC_SYS_TYPES_H

#include <stddef.h>
#include <stdint.h>

typedef long ssize_t;
typedef long off_t;
typedef int64_t off64_t;

typedef unsigned int mode_t;
typedef int pid_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef unsigned int id_t;

typedef uint64_t dev_t;
typedef uint64_t ino_t;
typedef uint64_t nlink_t;
typedef long blksize_t;
typedef long blkcnt_t;

typedef long time_t;
typedef long clock_t;
typedef int clockid_t;
typedef long suseconds_t;
typedef unsigned int useconds_t;

typedef unsigned int socklen_t;
typedef unsigned short sa_family_t;

#endif /* _RUSTLIBC_SYS_TYPES_H */
