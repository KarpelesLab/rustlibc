/* examples/hello.c — a freestanding C program linked against rustlibc.
 *
 * Exercises the whole path end to end: the rustlibc crt0 (`_start`) receives
 * control from the kernel, sets up argc/argv/envp, and calls main below; the
 * calls here go through rustlibc's own string/stdio/stdlib, not the system
 * libc. Build with `make hello` (see the Makefile).
 *
 * Note: this deliberately avoids printf, which is still a stub (variadic
 * support pending). It uses puts/fputs/fwrite, which are fully implemented. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char **argv)
{
	puts("Hello from rustlibc!");

	/* String ops. */
	char buf[64];
	strcpy(buf, "argv[0] = ");
	strcat(buf, argc > 0 ? argv[0] : "(none)");
	puts(buf);

	/* Allocation + memory ops. */
	char *heap = malloc(32);
	memset(heap, '=', 31);
	heap[31] = '\0';
	fputs(heap, stdout);
	fputc('\n', stdout);
	free(heap);

	/* Number conversion. */
	long n = strtol("  1234 rest", NULL, 10);
	char digits[8];
	int i = 8;
	digits[--i] = '\n';
	while (n > 0 && i > 0) {
		digits[--i] = '0' + (n % 10);
		n /= 10;
	}
	fwrite(digits + i, 1, 8 - i, stdout);

	return 0;
}
