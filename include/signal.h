/* signal.h — signals (C standard + POSIX).
 *
 * Part of rustlibc. Signal numbers, kill and raise are implemented; handler
 * installation (signal/sigaction) is a STUB pending a signal trampoline. */
#ifndef _RUSTLIBC_SIGNAL_H
#define _RUSTLIBC_SIGNAL_H

#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

#define SIGHUP 1
#define SIGINT 2
#define SIGQUIT 3
#define SIGILL 4
#define SIGTRAP 5
#define SIGABRT 6
#define SIGBUS 7
#define SIGFPE 8
#define SIGKILL 9
#define SIGUSR1 10
#define SIGSEGV 11
#define SIGUSR2 12
#define SIGPIPE 13
#define SIGALRM 14
#define SIGTERM 15
#define SIGCHLD 17
#define SIGCONT 18
#define SIGSTOP 19

typedef void (*sighandler_t)(int);
#define SIG_DFL ((sighandler_t)0)
#define SIG_IGN ((sighandler_t)1)
#define SIG_ERR ((sighandler_t)-1)

int kill(pid_t pid, int sig);
int raise(int sig);
sighandler_t signal(int signum, sighandler_t handler);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_SIGNAL_H */
