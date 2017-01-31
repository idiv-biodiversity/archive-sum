#include "util.h"
#include "archive-sum.h"

#include <stdio.h>
#include <sys/stat.h>

#define STDIN_BUF_SIZE 32768

int bsize(const char *filename, blksize_t *size) {
  if (filename == NULL) {
    *size = STDIN_BUF_SIZE;
  } else {
    struct stat s;

    if (stat(filename, &s) == -1) {
      perror(filename);
      return 0;
    }

    *size = s.st_blksize;
  }

  return 1;
}

void sanitize_filename(char *filename, char **open_filename, char **sanitized_filename) {
  if (filename == NULL || strcmp("-", filename) == 0) {
    *open_filename = NULL;
    *sanitized_filename = "stdin";
  } else {
    *open_filename = filename;
    *sanitized_filename = filename;
  }
}
