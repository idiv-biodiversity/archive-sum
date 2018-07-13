#!/usr/bin/env bash

# shellcheck source=test/setup-archive-sum-test.sh
source "$MESON_SOURCE_ROOT"/test/setup-archive-sum-test.sh || exit 1

APPEND_FILE=$(mktemp)
trap 'rm -r $APPEND_FILE $TMP_DIR' EXIT

md5sum "$TEST_ARCHIVE" > "$APPEND_FILE"

CMD="$ARCHIVE_SUM -c -a $APPEND_FILE $TEST_ARCHIVE"

$CMD &> /dev/null

# we don't diff the first line because tarball changes each time
# because files have different timestamps which are included in
# the tarball

function expected { cat << EOF
258622b1688250cb619f3c9ccaefb7eb  fbb/baz
c157a79031e1c40f85931829bc5fc552  fbb/bar
d3b07384d113edec49eaa6238ad5ff00  fbb/foo
EOF
}

echo "$CMD"
echo "---"
diff <(expected | sort) <(awk 'NR > 1' "$APPEND_FILE" | sort)
DIFF_EXIT=$?
echo "---"
echo "exit code: $DIFF_EXIT"
echo "---"

exit "$DIFF_EXIT"
