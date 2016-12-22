#include <stdlib.h>

#include <archive.h>
#include <archive_entry.h>

#include <openssl/md5.h>

#define BLOCK_SIZE 1048576

int archive_sum(const char *filename) {
  struct archive *a;
  struct archive_entry *e;

  int n, r;

  a = archive_read_new();

  archive_read_support_filter_all(a);
  archive_read_support_format_all(a);

  r = archive_read_open_filename(a, filename, BLOCK_SIZE);

  if (r != ARCHIVE_OK) {
    fprintf(stderr, "%s: %s\n", archive_error_string(a), filename);
    archive_read_free(a);
    return EXIT_FAILURE;
  }

  while (archive_read_next_header(a, &e) == ARCHIVE_OK) {
    if (archive_entry_filetype(e) != AE_IFREG)
      continue;

    la_ssize_t size;
    char buf[BLOCK_SIZE];
    MD5_CTX c;
    unsigned char md[16];

    MD5_Init(&c);

    while ((size = archive_read_data(a, &buf, BLOCK_SIZE)) > 0)
      MD5_Update(&c, buf, size);

    MD5_Final(md, &c);

    char *out = (char*)malloc(33);

    for (n = 0; n < 16; n++)
      snprintf(&(out[n*2]), 16*2, "%02x", (unsigned int)md[n]);

    printf("%s  %s\n", out, archive_entry_pathname(e));

    free(out);
  }

  r = archive_read_free(a);

  if (r != ARCHIVE_OK) {
    fprintf(stderr, "%s: %s\n", archive_error_string(a), filename);
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}

int main(int argc, char** argv) {
  int i;

  for (i = 1; i < argc; i++)
    archive_sum(argv[i]);

  return EXIT_SUCCESS;
}
