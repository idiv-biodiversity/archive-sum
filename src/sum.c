#ifdef HAVE_CONFIG_H
#include "config.h"
#endif

#include "archive-sum.h"

#include <archive.h>
#include <archive_entry.h>

int archive_sum(const EVP_MD *md, const char *filename) {
  struct archive *a;
  struct archive_entry *e;

  int r;
  unsigned int i, md_len;

  struct stat s;

  if (stat(filename, &s) == -1) {
    perror(filename);
    return EXIT_FAILURE;
  }

  const blksize_t bsize = s.st_blksize;

  a = archive_read_new();

  archive_read_support_filter_all(a);
  archive_read_support_format_all(a);

  r = archive_read_open_filename(a, filename, bsize);

  if (r != ARCHIVE_OK) {
    fprintf(stderr, "%s: %s\n", archive_error_string(a), filename);
    archive_read_free(a);
    return EXIT_FAILURE;
  }

  EVP_MD_CTX *mdctx = EVP_MD_CTX_create();
  unsigned char md_value[EVP_MAX_MD_SIZE];

  while (archive_read_next_header(a, &e) == ARCHIVE_OK) {
    if (archive_entry_filetype(e) != AE_IFREG)
      continue;

    ssize_t size;
    char buf[bsize];

    EVP_DigestInit_ex(mdctx, md, NULL);

    while ((size = archive_read_data(a, &buf, bsize)) > 0)
      EVP_DigestUpdate(mdctx, buf, size);

    EVP_DigestFinal_ex(mdctx, md_value, &md_len);

    for (i = 0; i < md_len; i++)
      printf("%02x", md_value[i]);

    printf("  %s\n", archive_entry_pathname(e));
  }

  EVP_MD_CTX_destroy(mdctx);

  r = archive_read_free(a);

  if (r != ARCHIVE_OK) {
    fprintf(stderr, "%s: %s\n", archive_error_string(a), filename);
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
