#include "archive-sum.h"

#include <getopt.h>
#include <stdbool.h>
#include <string.h>

// clang-format off

static const struct option long_options[] = {
  { "help",   no_argument,       0, 'h' },
  { "check",  optional_argument, 0, 'c' },
  { "digest", required_argument, 0, 'd' },
  { "quiet",  no_argument,       0,  0  },
  { "status", no_argument,       0,  0  },
  { 0, 0, 0, 0 }
};

// clang-format on

int main(int argc, char **argv) {
  int i, option_index;
  bool check = false, problems = false;
  char *check_dir = "";
  char *digest = "md5";
  verbosity_t verbosity = NORMAL;

  char usage[2048];
  snprintf(usage, 2048,
           "%s\n"
           "\n"
           "usage: %s [-c [dir]] [-d digest] archive...\n"
           "\n"
           "general options:\n"
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
           "  -h | --help          display help\n"
           "\n"
           "verification options:\n"
           "  -c | --check         read all files from archive and compare them\n"
           "                       with the content of their original files\n"
           "                       if an optional directory is given, files will\n"
           "                       be searched there instead of the current\n"
           "                       working directory\n"
           "\n"
           "  --quiet              don't print OK for each successfully verified file\n"
           "\n"
           "  --status             don't output anything, status code shows success\n"
           "\n",
           PACKAGE_STRING, argv[0], digest);

  // ---------------------------------------------------------------------------
  // command line options
  // ---------------------------------------------------------------------------

  while (1) {
    i = getopt_long(argc, argv, "c::d:h", long_options, &option_index);

    if (i == -1)
      break;

    switch (i) {
    case 0:
      if (strcmp("quiet", long_options[option_index].name) == 0)
        verbosity = QUIET;
      else if (strcmp("status", long_options[option_index].name) == 0)
        verbosity = STATUS;

      break;
    case 'c':
      check = true;
      if (optarg)
        check_dir = optarg;
      break;
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

  if (check) {

    for (i = optind; i < argc; i++)
      if (!archive_check(md, check_dir, argv[i], verbosity))
        problems = true;

  } else {

    for (i = optind; i < argc; i++)
      archive_sum(md, argv[i]);

  }

  // ---------------------------------------------------------------------------
  // exit
  // ---------------------------------------------------------------------------

  if (problems)
    return EXIT_FAILURE;
  else
    return EXIT_SUCCESS;
}
