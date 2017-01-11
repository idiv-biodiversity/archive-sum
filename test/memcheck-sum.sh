#!/bin/bash

if which valgrind &> /dev/null ; then
  source setup-archive-sum-test.sh || exit 1
  MEMCHECK_OUT=$(mktemp)
  trap "rm -fr $MEMCHECK_OUT $TMP_DIR" EXIT

  valgrind --tool=memcheck --leak-check=full --track-origins=yes \
    $ARCHIVE_SUM $TEST_ARCHIVE \
      &> $MEMCHECK_OUT

  cat $MEMCHECK_OUT
  grep -q 'ERROR SUMMARY: 0' $MEMCHECK_OUT
else
  echo "no valgrind, skipping test"
  exit 77
fi
