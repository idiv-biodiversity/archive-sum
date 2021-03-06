#include "archive-sum.h"

#include <getopt.h>
#include <stdbool.h>

// clang-format off

static const struct option long_options[] = {
  { "help",    no_argument,       0, 'h' },
  { "version", no_argument,       0,  0  },
  { "append",  required_argument, 0, 'a' },
  { "check",   optional_argument, 0, 'c' },
  { "digest",  required_argument, 0, 'd' },
  { "quiet",   no_argument,       0,  0  },
  { "status",  no_argument,       0,  0  },
  { 0, 0, 0, 0 }
};

// clang-format on

int main(int argc, char **argv) {
  int i, option_index;
  bool check = false, problems = false;
  char *append = NULL;
  char *check_dir = "";
  char *digest = "md5";
  verbosity_t verbosity = NORMAL;

  char usage[2048];
  snprintf(usage, 2048,
           "archive-sum %s\n"
           "\n"
           "USAGE\n"
           "\n"
           "  archive-sum [options] archive...\n"
           "\n"
           "ARGUMENTS\n"
           "\n"
           "  archive...           list of archive files\n"
           "\n"
           "                       if no archive is given or archive is \"-\"\n"
           "                       read from stdin\n"
           "\n"
           "                       for a full list of supported formats:\n"
           "                         man 5 libarchive-formats\n"
           "\n"
           "OPTIONS\n"
           "\n"
           "  -d, --digest d       choose digest: md5, sha1, sha256, sha512, ...\n"
           "\n"
           "                       for a full list of supported digests:\n"
           "                         openssl list-message-digest-algorithms\n"
           "\n"
           "                       defaults to %s\n"
           "\n"
           "  -c, --check [dir]    read all files from archive and compare them\n"
           "                       with the content of their original files\n"
           "                       if an optional directory is given, files will\n"
           "                       be searched there instead of the current\n"
           "                       working directory\n"
           "\n"
           "  -a, --append file    append digests from files in archive to file\n"
           "                       only applies with -c or --check\n"
           "\n"
           "  --quiet              don't print OK for each successfully verified file\n"
           "\n"
           "  --status             don't output anything, status code shows success\n"
           "\n"
           "OTHER OPTIONS\n"
           "\n"
           "  -h, --help           display this help\n"
           "\n"
           "  --version            display version\n"
           "\n",
           PACKAGE_VERSION, digest);

  // ---------------------------------------------------------------------------
  // command line options
  // ---------------------------------------------------------------------------

  while (1) {
    i = getopt_long(argc, argv, "a:c::d:h", long_options, &option_index);

    if (i == -1)
      break;

    switch (i) {
    case 0:
      if (strcmp("quiet", long_options[option_index].name) == 0)
        verbosity = QUIET;
      else if (strcmp("version", long_options[option_index].name) == 0) {
        printf("archive-sum %s\n", PACKAGE_VERSION);
        return EXIT_SUCCESS;
      } else if (strcmp("status", long_options[option_index].name) == 0)
        verbosity = STATUS;

      break;
    case 'a':
      append = optarg;
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

  if (argc == optind) {

    // no archive file given, read from stdin

    if (check) {
      if (!archive_check(md, check_dir, NULL, append, verbosity))
        problems = true;
    } else {
      archive_sum(md, NULL);
    }

  } else {

    // iterate through archive files

    if (check) {

      for (i = optind; i < argc; i++)
        if (!archive_check(md, check_dir, argv[i], append, verbosity))
          problems = true;

    } else {

      for (i = optind; i < argc; i++)
        archive_sum(md, argv[i]);
    }
  }

  // ---------------------------------------------------------------------------
  // exit
  // ---------------------------------------------------------------------------

  if (problems)
    return EXIT_FAILURE;
  else
    return EXIT_SUCCESS;
}
