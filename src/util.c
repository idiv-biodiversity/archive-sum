#include "util.h"

#include <sys/stat.h>
#include <stdio.h>

int bsize(const char *file, blksize_t *size) {
  struct stat s;

  if (stat(file, &s) == -1) {
    perror(file);
    return 0;
  }

  *size = s.st_blksize;

  return 1;
}
