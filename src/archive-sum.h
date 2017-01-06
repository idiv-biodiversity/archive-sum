#ifndef ARCHIVE_SUM_H
#define ARCHIVE_SUM_H

#include <openssl/evp.h>

typedef enum { NORMAL, QUIET, STATUS } verbosity_t;

int archive_check(const EVP_MD *md, const char *check_dir, const char *archive,
                  verbosity_t verbosity);

int archive_sum(const EVP_MD *md, const char *filename);

#endif
