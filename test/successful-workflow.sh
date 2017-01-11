#!/bin/bash

source setup-archive-sum-test.sh || exit 1
trap "rm -fr $TMP_DIR" EXIT

$ARCHIVE_SUM -c $TEST_ARCHIVE
