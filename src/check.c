#include "archive-sum.h"
#include "util.h"

#include <fcntl.h>

#include <linux/limits.h>

#include <archive.h>
#include <archive_entry.h>

#include <sys/stat.h>
#include <sys/types.h>

int archive_check(const EVP_MD *md, const char *check_dir, char *filename,
                  const verbosity_t verbosity) {

  struct archive *a;
  struct archive_entry *e;
  const char *e_pathname;

  unsigned int archive_md_len, original_md_len, missing = 0, warning = 0;

  int original_fd;

  blksize_t archive_bsize, original_bsize;

  ssize_t size;

  char original_path[PATH_MAX];

  // new archive
  a = archive_read_new();
  archive_read_support_filter_all(a);
  archive_read_support_format_all(a);

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
  unsigned char md_value_archive[EVP_MAX_MD_SIZE];
  unsigned char md_value_original[EVP_MAX_MD_SIZE];

  // read through archive entries
  while (archive_read_next_header(a, &e) == ARCHIVE_OK) {
    // regular files only
    if (archive_entry_filetype(e) != AE_IFREG)
      continue;

    e_pathname = archive_entry_pathname(e);

    // calculate digest (archive entry)
    EVP_DigestInit_ex(mdctx, md, NULL);

    while ((size = archive_read_data(a, &buf, archive_bsize)) > 0)
      EVP_DigestUpdate(mdctx, buf, size);

    EVP_DigestFinal_ex(mdctx, md_value_archive, &archive_md_len);

    // read original file
    if (strlen(check_dir) == 0) {
      snprintf(original_path, PATH_MAX, "%s", e_pathname);
    } else {
      snprintf(original_path, PATH_MAX, "%s/%s", check_dir, e_pathname);
    }

    if (!bsize(original_path, &original_bsize)) {
      missing += 1;
      continue;
    }

    original_fd = open(original_path, O_RDONLY);

    // calculate digest (original file)
    EVP_DigestInit_ex(mdctx, md, NULL);

    while ((size = read(original_fd, buf, original_bsize)) > 0)
      EVP_DigestUpdate(mdctx, buf, size);

    close(original_fd);
    EVP_DigestFinal_ex(mdctx, md_value_original, &original_md_len);

    if (archive_md_len != original_md_len) {
      fprintf(stderr, "%s: digests don't have the same length\n", e_pathname);
      continue;
    }

    // compare digests
    if (verbosity == NORMAL) {

      printf("%s: ", e_pathname);
      if (memcmp(md_value_archive, md_value_original, archive_md_len) == 0) {
        printf("OK\n");
      } else {
        warning += 1;
        printf("FAILED\n");
      }

    } else if (verbosity == QUIET) {

      if (memcmp(md_value_archive, md_value_original, archive_md_len) != 0) {
        warning += 1;
        printf("%s: FAILED\n", e_pathname);
      }

    } else if (verbosity == STATUS) {

      if (memcmp(md_value_archive, md_value_original, archive_md_len) != 0)
        warning += 1;
    }
  }

  // free digest
  EVP_MD_CTX_destroy(mdctx);

  // free archive
  if (archive_read_free(a) != ARCHIVE_OK) {
    fprintf(stderr, "%s: %s\n", archive_error_string(a), error_filename);
    return 0;
  }

  // issue warning summaries
  if (missing > 0 && verbosity != STATUS) {
    fprintf(stderr, "%s: WARNING: %u listed %s could not be read\n", error_filename, missing,
            warning == 1 ? "file" : "files");
  }

  if (warning > 0 && verbosity != STATUS) {
    fprintf(stderr, "%s: WARNING: %u computed %s did NOT match\n", error_filename, warning,
            warning == 1 ? "checksum" : "checksums");
  }

  if (missing > 0 || warning > 0)
    return 0;

  return 1;
}
