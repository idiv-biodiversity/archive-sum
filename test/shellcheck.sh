#!/usr/bin/env bash

if command -v shellcheck &> /dev/null ; then
  cd "$MESON_SOURCE_ROOT" || exit 1
  shellcheck test/*.sh
else
  echo "no shellcheck, skipping test"
  exit 77
fi
