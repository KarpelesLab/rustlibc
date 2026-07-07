/* unistd.h — POSIX standard symbolic constants and system calls.
 *
 * Part of rustlibc. The declared syscalls are implemented. */
#ifndef _RUSTLIBC_UNISTD_H
#define _RUSTLIBC_UNISTD_H

#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

#define STDIN_FILENO 0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

ssize_t read(int fd, void *buf, size_t count);
ssize_t write(int fd, const void *buf, size_t count);
int close(int fd);
off_t lseek(int fd, off_t offset, int whence);
pid_t getpid(void);
_Noreturn void _exit(int status);
unsigned int sleep(unsigned int seconds);
int usleep(unsigned int usec);
char *getcwd(char *buf, size_t size);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_UNISTD_H */
