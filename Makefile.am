EXTRA_DIST = autogen.sh LICENSE README.md

bin_PROGRAMS = archive-sum

archive_sum_CPPFLAGS = $(CRYPTO_CFLAGS) $(LIBARCHIVE_CFLAGS)
archive_sum_LDADD = $(CRYPTO_LIBS) $(LIBARCHIVE_LIBS)

archive_sum_SOURCES = \
	archive-sum.c
