#!/usr/bin/env bash

export ARCHIVE_SUM=$PWD/src/archive-sum

TMP_DIR=$(mktemp -d)
export TMP_DIR

cd "$TMP_DIR" || exit 1

export TEST_ARCHIVE_DIR_NAME=fbb
export TEST_ARCHIVE_DIR=$TEST_ARCHIVE_DIR_NAME
export TEST_ARCHIVE=$TEST_ARCHIVE_DIR_NAME.tar.gz

# create archive
mkdir $TEST_ARCHIVE_DIR
echo foo > $TEST_ARCHIVE_DIR/foo
echo bar > $TEST_ARCHIVE_DIR/bar
echo baz > $TEST_ARCHIVE_DIR/baz
bsdtar czf $TEST_ARCHIVE $TEST_ARCHIVE_DIR_NAME
