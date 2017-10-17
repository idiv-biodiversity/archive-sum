#!/usr/bin/env bash

if command -v cppcheck &> /dev/null ; then
  cppcheck \
    --std=c99 \
    --enable=all \
    --error-exitcode=1 \
    --suppress=missingIncludeSystem \
    -I. \
    "$MESON_SOURCE_ROOT/src"
else
  echo "no cppcheck, skipping test"
  exit 77
fi
