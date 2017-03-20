#include "archive-sum.h"
#include "util.h"

#include <archive.h>
#include <archive_entry.h>

int archive_sum(const EVP_MD *md, char *filename) {
  struct archive *a;
  struct archive_entry *e;

  unsigned int i, md_len;

  ssize_t size;

  blksize_t archive_bsize;

  // new archive
  a = archive_read_new();
  archive_read_support_filter_all(a);
  archive_read_suppot_format_all(a);

  // sanitize filename for opening and for error messages
  char *error_filename, *open_filename;
  sanitize_filename(filename, &open_filename, &error_filename);

  // get fs bsize for archive
  if (!bsize(open_filename, &archive_bsize))
    return 0;

  // open archive
  if (archive_read_open_filename(a, open_filename, archive_bsize) != ARCHIVE_OK) {
    fprintf(stderr, "%s: %s\n", archive_error_string(a), error_filename);
    archive_read_free(a);
    return 0;
  }

  char buf[archive_bsize];

  // init digest
  EVP_MD_CTX *mdctx = EVP_MD_CTX_create();
  unsigned char md_value[EVP_MAX_MD_SIZE];

  // read through archive entries
  while (archive_read_next_header(a, &e) == ARCHIVE_OK) {
    // regular files only
    if (archive_entry_filetype(e) != AE_IFREG)
      continue;

    // calculate digest
    EVP_DigestInit_ex(mdctx, md, NULL);

    while ((size = archive_read_data(a, &buf, archive_bsize)) > 0)
      EVP_DigestUpdate(mdctx, buf, size);

    EVP_DigestFinal_ex(mdctx, md_value, &md_len);

    // print digest
    for (i = 0; i < md_len; i++)
      printf("%02x", md_value[i]);

    printf("  %s\n", archive_entry_pathname(e));
  }

  // free digest
  EVP_MD_CTX_destroy(mdctx);

  // free archive
  if (archive_read_free(a) != ARCHIVE_OK) {
    fprintf(stderr, "%s: %s\n", archive_error_string(a), error_filename);
    return 0;
  }

  return 1;
}
