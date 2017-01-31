#!/bin/bash

source setup-archive-sum-test.sh || exit 1
trap "rm -fr $TMP_DIR" EXIT

dd if=$TEST_ARCHIVE bs=1M 2> /dev/null | $ARCHIVE_SUM -c
