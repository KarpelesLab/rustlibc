/* examples/hello.c — a freestanding C program linked against rustlibc.
 *
 * Exercises the whole path end to end: the rustlibc crt0 (`_start`) receives
 * control from the kernel, sets up argc/argv/envp, and calls main below; the
 * calls here go through rustlibc's own string/stdio/stdlib, not the system
 * libc. Build with `make hello` (see the Makefile).
 *
 * Uses printf, puts, malloc, and the string/number helpers — all going through
 * rustlibc rather than the system libc. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char **argv)
{
	puts("Hello from rustlibc!");

	/* Formatted output through rustlibc's own printf engine. */
	printf("argc=%d, argv[0]=%s\n", argc, argc > 0 ? argv[0] : "(none)");
	printf("int=%d hex=%#x oct=%o char=%c\n", 42, 255, 64, '!');
	printf("padded=[%6d] left=[%-6d] zero=[%06d]\n", 7, 7, 7);
	printf("float=%.3f str=%.4s pct=%%\n", 3.14159, "truncated");

	/* Allocation + memory ops. */
	char *heap = malloc(32);
	memset(heap, '=', 31);
	heap[31] = '\0';
	printf("heap: %s\n", heap);
	free(heap);

	/* snprintf into a fixed buffer. */
	char buf[32];
	int n = snprintf(buf, sizeof(buf), "%d + %d = %d", 2, 3, 2 + 3);
	printf("snprintf wrote %d bytes: \"%s\"\n", n, buf);

	/* Number conversion round-trip. */
	long v = strtol("  1234 rest", NULL, 10);
	printf("strtol -> %ld\n", v);

	return 0;
}
