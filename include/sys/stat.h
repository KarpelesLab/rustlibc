/* sys/stat.h — file status (POSIX). Part of rustlibc.
 *
 * struct stat has a different layout per architecture; it must match
 * src/sys/stat.rs exactly. */
#ifndef _RUSTLIBC_SYS_STAT_H
#define _RUSTLIBC_SYS_STAT_H

#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

#if defined(__x86_64__)
struct stat {
	dev_t st_dev;
	ino_t st_ino;
	nlink_t st_nlink;
	mode_t st_mode;
	uid_t st_uid;
	gid_t st_gid;
	unsigned int __pad0;
	dev_t st_rdev;
	off_t st_size;
	long st_blksize;
	long st_blocks;
	time_t st_atime;
	long st_atime_nsec;
	time_t st_mtime;
	long st_mtime_nsec;
	time_t st_ctime;
	long st_ctime_nsec;
	long __unused[3];
};
#elif defined(__aarch64__)
struct stat {
	dev_t st_dev;
	ino_t st_ino;
	mode_t st_mode;
	unsigned int st_nlink;
	uid_t st_uid;
	gid_t st_gid;
	dev_t st_rdev;
	dev_t __pad1;
	off_t st_size;
	int st_blksize;
	int __pad2;
	long st_blocks;
	time_t st_atime;
	long st_atime_nsec;
	time_t st_mtime;
	long st_mtime_nsec;
	time_t st_ctime;
	long st_ctime_nsec;
	unsigned int __unused[2];
};
#else
#error "rustlibc: struct stat is not defined for this architecture"
#endif

#define S_IFMT 0170000
#define S_IFSOCK 0140000
#define S_IFLNK 0120000
#define S_IFREG 0100000
#define S_IFBLK 0060000
#define S_IFDIR 0040000
#define S_IFCHR 0020000
#define S_IFIFO 0010000
#define S_ISUID 0004000
#define S_ISGID 0002000
#define S_ISVTX 0001000
#define S_IRWXU 0000700
#define S_IRWXG 0000070
#define S_IRWXO 0000007

#define S_ISREG(m) (((m) & S_IFMT) == S_IFREG)
#define S_ISDIR(m) (((m) & S_IFMT) == S_IFDIR)
#define S_ISCHR(m) (((m) & S_IFMT) == S_IFCHR)
#define S_ISBLK(m) (((m) & S_IFMT) == S_IFBLK)
#define S_ISFIFO(m) (((m) & S_IFMT) == S_IFIFO)
#define S_ISLNK(m) (((m) & S_IFMT) == S_IFLNK)
#define S_ISSOCK(m) (((m) & S_IFMT) == S_IFSOCK)

int stat(const char *path, struct stat *buf);
int lstat(const char *path, struct stat *buf);
int fstat(int fd, struct stat *buf);
int fstatat(int dirfd, const char *path, struct stat *buf, int flags);
int mkdir(const char *path, mode_t mode);
int chmod(const char *path, mode_t mode);
int fchmod(int fd, mode_t mode);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_SYS_STAT_H */
