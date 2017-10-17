#!/usr/bin/env bash

# shellcheck source=test/setup-archive-sum-test.sh
source "$MESON_SOURCE_ROOT"/test/setup-archive-sum-test.sh || exit 1

ARCHIVE_CHECK_OUTPUT=$(mktemp)
trap 'rm -r $ARCHIVE_CHECK_OUTPUT $TMP_DIR' EXIT

# corrupt original
echo foo >> $TEST_ARCHIVE_DIR/foo

CMD="$ARCHIVE_SUM -c --status $TEST_ARCHIVE"

$CMD &> "$ARCHIVE_CHECK_OUTPUT"
ARCHIVE_CHECK_EXIT=$?

echo "$CMD"
echo "---"
cat "$ARCHIVE_CHECK_OUTPUT"
echo "---"
echo "exit code: $ARCHIVE_CHECK_EXIT"
echo "---"

[[ $ARCHIVE_CHECK_EXIT -ne 0 ]] &&
[[ ! -s $ARCHIVE_CHECK_OUTPUT ]]
