#ifndef ARCHIVE_SUM_UTIL_H
#define ARCHIVE_SUM_UTIL_H

#include <sys/types.h>

int bsize(const char *file, blksize_t *size);

void sanitize_filename(char *filename, char **open_filename, char **sanitized_filename);

#endif
