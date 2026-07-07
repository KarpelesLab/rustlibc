/* poll.h — synchronous I/O multiplexing (POSIX). Part of rustlibc. */
#ifndef _RUSTLIBC_POLL_H
#define _RUSTLIBC_POLL_H

#ifdef __cplusplus
extern "C" {
#endif

typedef unsigned long nfds_t;

#define POLLIN 0x001
#define POLLPRI 0x002
#define POLLOUT 0x004
#define POLLERR 0x008
#define POLLHUP 0x010
#define POLLNVAL 0x020

struct pollfd {
	int fd;
	short events;
	short revents;
};

int poll(struct pollfd *fds, nfds_t nfds, int timeout);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_POLL_H */
