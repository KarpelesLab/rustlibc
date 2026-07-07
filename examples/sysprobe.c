/* examples/sysprobe.c — exercises the newer syscall wrappers in rustlibc:
 * uname, stat, and the opendir/readdir directory API. Freestanding, linked
 * against rustlibc only (build with `make sysprobe`). */
#include <dirent.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/utsname.h>

int main(void)
{
	struct utsname u;
	if (uname(&u) == 0)
		printf("uname: %s %s (%s)\n", u.sysname, u.release, u.machine);
	else
		puts("uname failed");

	struct stat st;
	if (stat("/etc/hostname", &st) == 0)
		printf("/etc/hostname: %ld bytes, mode %o\n",
		       (long)st.st_size, st.st_mode & 07777);
	else
		puts("stat(/etc/hostname) failed (may not exist)");

	/* List up to 8 entries of the current directory. */
	DIR *d = opendir(".");
	if (d) {
		puts("entries in \".\":");
		struct dirent *e;
		int count = 0;
		while ((e = readdir(d)) != NULL && count < 8) {
			printf("  %s%s\n", e->d_name,
			       e->d_type == DT_DIR ? "/" : "");
			count++;
		}
		closedir(d);
	} else {
		puts("opendir(\".\") failed");
	}
	return 0;
}
