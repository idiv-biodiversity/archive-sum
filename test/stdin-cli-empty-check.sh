#!/usr/bin/env bash

# shellcheck source=test/setup-archive-sum-test.sh
source "$MESON_SOURCE_ROOT"/test/setup-archive-sum-test.sh || exit 1

trap 'rm -r $TMP_DIR' EXIT

dd if="$TEST_ARCHIVE" bs=1M 2> /dev/null | "$ARCHIVE_SUM" -c
