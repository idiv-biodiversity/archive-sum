#!/usr/bin/env bash

# shellcheck source=test/setup-archive-sum-test.sh
source "$MESON_SOURCE_ROOT"/test/setup-archive-sum-test.sh || exit 1

trap 'rm -r $TMP_DIR' EXIT

# corrupt original
echo foo >> $TEST_ARCHIVE_DIR/foo

# expect failure
$ARCHIVE_SUM -c $TEST_ARCHIVE |& grep -q '^fbb/foo: FAILED$'
