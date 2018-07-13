#!/usr/bin/env bash

if command -v valgrind &> /dev/null ; then
  # shellcheck source=test/setup-archive-sum-test.sh
  source "$MESON_SOURCE_ROOT"/test/setup-archive-sum-test.sh || exit 1
  MEMCHECK_OUT=$(mktemp)
  APPEND_FILE=$(mktemp)
  trap 'rm -r $MEMCHECK_OUT $APPEND_FILE $TMP_DIR' EXIT

  valgrind --tool=memcheck --leak-check=full --track-origins=yes \
    "$ARCHIVE_SUM" -c -a "$APPEND_FILE" "$TEST_ARCHIVE" \
      &> "$MEMCHECK_OUT"

  cat "$MEMCHECK_OUT"
  grep -q 'ERROR SUMMARY: 0' "$MEMCHECK_OUT"
else
  echo "no valgrind, skipping test"
  exit 77
fi
