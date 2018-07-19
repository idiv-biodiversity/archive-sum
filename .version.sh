#!/bin/sh

version=$(git describe --always --dirty 2> /dev/null | sed 's/^v//')

if [ -n "$version" ] ; then
  echo "$version"
else
  echo "1.1.1"
fi
