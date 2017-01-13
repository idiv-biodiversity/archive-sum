#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.63])
AC_INIT([archive-sum], [0.1.0], [https://github.com/idiv-biodiversity/archive-sum/issues])
AC_CONFIG_SRCDIR([src/archive-sum.c])
AC_CONFIG_HEADERS([config.h])

# Automake invocation.
AM_INIT_AUTOMAKE([foreign])

# Checks for programs.
AC_PROG_CC_C99

# Checks for libraries.
PKG_CHECK_MODULES(CRYPTO, libcrypto)
PKG_CHECK_MODULES(LIBARCHIVE, libarchive)

# Checks for header files.
AC_CHECK_HEADERS([fcntl.h string.h])

# Checks for typedefs, structures, and compiler characteristics.
# add when travis has autoconf 2.69+ AC_CHECK_HEADER_STDBOOL
AC_HEADER_STDBOOL
AC_TYPE_SSIZE_T
AC_CHECK_MEMBERS([struct stat.st_blksize])

# Checks for operating system services or capabilities.
AC_SYS_LARGEFILE
AM_CONDITIONAL([WANTS_LARGEFILE], [test "$enable_largefile" != no])

# Checks for library functions.

# Optional stuff.
AC_ARG_ENABLE([all_tests],
              [AS_HELP_STRING([--enable-all-tests], [enable long running tests])],
              [],
              [all_tests=no])
AM_CONDITIONAL([WANTS_ALL_TESTS], [test "$enable_all_tests" == yes])

AC_CONFIG_FILES([Makefile
                 src/Makefile
                 test/Makefile
                 test/cppcheck.sh
                 test/setup-archive-sum-test.sh])
AC_OUTPUT
