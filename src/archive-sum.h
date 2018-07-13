#ifndef ARCHIVE_SUM_H
#define ARCHIVE_SUM_H

#include "config.h"

#include <openssl/evp.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>

typedef enum { NORMAL, QUIET, STATUS } verbosity_t;

int archive_check(const EVP_MD *md, const char *check_dir, char *archive,
                  const char *append, const verbosity_t verbosity);

int archive_sum(const EVP_MD *md, char *filename);

#endif
