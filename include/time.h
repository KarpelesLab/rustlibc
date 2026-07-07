/* time.h — time and date (C standard + POSIX clocks).
 *
 * Part of rustlibc. clock_gettime and time are implemented; calendar
 * conversions are STUBs (see src/time.rs). */
#ifndef _RUSTLIBC_TIME_H
#define _RUSTLIBC_TIME_H

#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

#define CLOCK_REALTIME 0
#define CLOCK_MONOTONIC 1
#define CLOCK_PROCESS_CPUTIME_ID 2
#define CLOCK_THREAD_CPUTIME_ID 3

struct timespec {
	time_t tv_sec;
	long tv_nsec;
};

struct tm {
	int tm_sec;
	int tm_min;
	int tm_hour;
	int tm_mday;
	int tm_mon;
	int tm_year;
	int tm_wday;
	int tm_yday;
	int tm_isdst;
	long tm_gmtoff;
	const char *tm_zone;
};

int clock_gettime(clockid_t clk_id, struct timespec *tp);
time_t time(time_t *tloc);
struct tm *gmtime_r(const time_t *timep, struct tm *result);

#ifdef __cplusplus
}
#endif

#endif /* _RUSTLIBC_TIME_H */
