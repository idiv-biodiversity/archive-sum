NORMAL_TESTS = \
	cppcheck.sh \
	corrupted-workflow.sh \
	successful-workflow.sh \
	check-missing.sh \
	check-quiet.sh \
	check-status.sh \
	memcheck-check.sh \
	memcheck-sum.sh

LARGEFILE_TESTS = \
	large-file-support-check.sh \
	large-file-support-sum.sh

TESTS = $(NORMAL_TESTS)

LONG_TESTS =

if WANTS_LARGEFILE
LONG_TESTS += $(LARGEFILE_TESTS)
endif

if WANTS_ALL_TESTS
TESTS += $(LONG_TESTS)
endif

EXTRA_DIST = $(NORMAL_TESTS) $(LARGEFILE_TESTS)

TEST_EXTENSIONS = .sh
SH_LOG_COMPILER = bash
