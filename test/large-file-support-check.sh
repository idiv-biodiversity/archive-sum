#!/bin/bash

# without LFS: up to 2^31 bytes, i.e. 2 GiB, or 2048 MiB
# so we take 2049 MiB chunks
BYTES=2148532224
CHUNK_SIZE=1M
NCHUNKS=2049

# we need twice that size of free space in the current file system
# or else we can't perform this check

FREE_BYTES=$(df -B1 . | awk 'NR == 2 { print $4 }')

if [[ $FREE_BYTES -gt $(( $BYTES * 2 )) ]] ; then
  LARGE_FILE=$(mktemp -p .)
  TEST_ARCHIVE=$(mktemp -p .)
  trap "rm -f $LARGE_FILE $TEST_ARCHIVE" EXIT

  # populate a large file with random data
  dd if=/dev/urandom bs=$CHUNK_SIZE count=$NCHUNKS of=$LARGE_FILE

  bsdtar cf $TEST_ARCHIVE $LARGE_FILE

  ../src/archive-sum -c $TEST_ARCHIVE
else
  echo "not enough disk space, skipping test"
  exit 77
fi
