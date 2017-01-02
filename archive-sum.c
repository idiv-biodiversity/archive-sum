#include <getopt.h>
#include <stdlib.h>

#include <archive.h>
#include <archive_entry.h>

#include <openssl/evp.h>

#ifdef HAVE_CONFIG_H
#include "config.h"
#endif

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

    la_ssize_t size;
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

static const struct option long_options[] = {
  { "help",   no_argument,       0, 'h' },
  { "digest", required_argument, 0, 'd' },
  { 0, 0, 0, 0 }
};

int main(int argc, char **argv) {
  int i;
  char *digest = "md5";

  char usage [1024];
  snprintf(usage,
           1024,
           "%s\n"
           "\n"
           "usage: %s [-d digest] archive...\n"
           "\n"
           "options:\n"
           "  -d | --digest        choose digest: md5, sha1, sha256, sha512, ...\n"
           "\n"
           "                       for a full list of supported digests:\n"
           "                         openssl list-message-digest-algorithms\n"
           "\n"
           "                       default is %s\n"
           "\n"
           "  archive...           list of archive files\n"
           "\n"
           "                       for a full list of supported formats:\n"
           "                         man 5 libarchive-formats\n"
           "\n"
           "  -h | --help          display help\n",
           PACKAGE_STRING,
           argv[0],
           digest
           );

  // ---------------------------------------------------------------------------
  // command line options
  // ---------------------------------------------------------------------------

  while (1) {
    i = getopt_long(argc, argv, "d:h", long_options, 0);

    if (i == -1)
      break;

    switch (i) {
    case 'd':
      digest = optarg;
      break;
    case 'h':
      printf(usage);
      return EXIT_SUCCESS;
    default:
      fprintf(stderr, "---\n%s", usage);
      return EXIT_FAILURE;
    }
  }

  if (argc == optind) {
    fprintf(stderr, "%s: no archive file specified\n---\n%s", argv[0], usage);
    return EXIT_FAILURE;
  }

  // ---------------------------------------------------------------------------
  // digest
  // ---------------------------------------------------------------------------

  OpenSSL_add_all_digests();
  const EVP_MD *md = EVP_get_digestbyname(digest);
  EVP_cleanup();

  if (!md) {
    fprintf(stderr, "%s: unknown digest: %s\n---\n%s", argv[0], digest, usage);
    return EXIT_FAILURE;
  }

  // ---------------------------------------------------------------------------
  // handle archives
  // ---------------------------------------------------------------------------

  for (i = optind; i < argc; i++)
    archive_sum(md, argv[i]);

  return EXIT_SUCCESS;
}
