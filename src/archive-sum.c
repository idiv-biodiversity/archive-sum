#ifdef HAVE_CONFIG_H
#include "config.h"
#endif

#include "archive-sum.h"

#include <getopt.h>

// clang-format off

static const struct option long_options[] = {
  { "help",   no_argument,       0, 'h' },
  { "digest", required_argument, 0, 'd' },
  { 0, 0, 0, 0 }
};

// clang-format on

int main(int argc, char **argv) {
  int i;
  char *digest = "md5";

  char usage[1024];
  snprintf(usage, 1024,
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
           PACKAGE_STRING, argv[0], digest);

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
      printf("%s", usage);
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
