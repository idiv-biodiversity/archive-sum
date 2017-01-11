#!/bin/bash

source setup-archive-sum-test.sh || exit 1
trap "rm -fr $TMP_DIR" EXIT

# corrupt original
echo foo >> $TEST_ARCHIVE_DIR/foo

# expect failure
$ARCHIVE_SUM -c $TEST_ARCHIVE |& grep -q '^fbb/foo: FAILED$'
