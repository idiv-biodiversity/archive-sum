#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.63])
AC_INIT([archive-sum], [0.1.0], [https://github.com/idiv-biodiversity/archive-sum/issues])
AC_CONFIG_SRCDIR([archive-sum.c])
AC_CONFIG_HEADERS([config.h])

# Automake invocation.
AM_INIT_AUTOMAKE([foreign])

# Checks for programs.
AC_PROG_CC_C99

# Checks for libraries.
PKG_CHECK_MODULES(CRYPTO, libcrypto)
PKG_CHECK_MODULES(LIBARCHIVE, libarchive)

# Checks for header files.
AC_CHECK_HEADERS([stdlib.h])

# Checks for typedefs, structures, and compiler characteristics.
AC_TYPE_SSIZE_T
AC_CHECK_MEMBERS([struct stat.st_blksize])

# Checks for library functions.

AC_CONFIG_FILES([Makefile])
AC_OUTPUT
