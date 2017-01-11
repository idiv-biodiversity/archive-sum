#!/bin/bash

source setup-archive-sum-test.sh || exit 1
ARCHIVE_CHECK_OUTPUT=$(mktemp)
trap "rm -fr $ARCHIVE_CHECK_OUTPUT $TMP_DIR" EXIT

# corrupt original
rm -f $TEST_ARCHIVE_DIR/bar

CMD="$ARCHIVE_SUM -c $TEST_ARCHIVE"

$CMD &> $ARCHIVE_CHECK_OUTPUT
ARCHIVE_CHECK_EXIT=$?

echo $CMD
echo "---"
cat $ARCHIVE_CHECK_OUTPUT
echo "---"
echo "exit code: $ARCHIVE_CHECK_EXIT"
echo "---"

[[ $ARCHIVE_CHECK_EXIT -ne 0 ]] &&
grep -qE 'WARNING:.+could not be read' $ARCHIVE_CHECK_OUTPUT
