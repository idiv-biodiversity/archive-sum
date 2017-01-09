#ifndef ARCHIVE_SUM_H
#define ARCHIVE_SUM_H

#include <openssl/evp.h>

int archive_sum(const EVP_MD *md, const char *filename);

#endif
