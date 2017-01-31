#!/bin/bash

source setup-archive-sum-test.sh || exit 1
trap "rm -fr $TMP_DIR" EXIT

mkdir migrate
mv $TEST_ARCHIVE_DIR migrate

$ARCHIVE_SUM -cmigrate $TEST_ARCHIVE
