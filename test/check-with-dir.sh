#!/usr/bin/env bash

# shellcheck source=test/setup-archive-sum-test.sh
source "$MESON_SOURCE_ROOT"/test/setup-archive-sum-test.sh || exit 1

trap 'rm -r $TMP_DIR' EXIT

mkdir migrate
mv $TEST_ARCHIVE_DIR migrate

$ARCHIVE_SUM -cmigrate $TEST_ARCHIVE
