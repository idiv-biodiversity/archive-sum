#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.69])
AC_INIT([archive-sum], [0.1.0], [https://github.com/wookietreiber/archive-sum/issues])
AC_CONFIG_SRCDIR([archive-sum.c])
AC_CONFIG_HEADERS([config.h])

# Automake invocation.
AM_INIT_AUTOMAKE([foreign])

# Checks for programs.
AC_PROG_CC

# Checks for libraries.
PKG_CHECK_MODULES(CRYPTO, libcrypto)
PKG_CHECK_MODULES(LIBARCHIVE, libarchive)

# Checks for header files.
AC_CHECK_HEADERS([stdlib.h])

# Checks for typedefs, structures, and compiler characteristics.

# Checks for library functions.
AC_FUNC_MALLOC

AC_CONFIG_FILES([Makefile])

AC_OUTPUT
