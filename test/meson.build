check_missing = find_program('check-missing.sh')
test(
  'check-missing', check_missing,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()]
)

check_quiet = find_program('check-quiet.sh')
test(
  'check-quiet', check_quiet,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()]
)

check_status = find_program('check-status.sh')
test(
  'check-status', check_status,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()]
)

check_with_dir = find_program('check-with-dir.sh')
test(
  'check-with-dir', check_with_dir,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()]
)

corrupted_workflow = find_program('corrupted-workflow.sh')
test(
  'corrupted-workflow', corrupted_workflow,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()]
)

cppcheck = find_program('cppcheck.sh')
test(
  'cppcheck', cppcheck,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()],
  suite: ['style']
)

lfs_check = find_program('lfs-check.sh')
test(
  'lfs-check', lfs_check,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()],
  suite: ['lfs']
)

lfs_sum = find_program('lfs-sum.sh')
test(
  'lfs-sum', lfs_sum,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()],
  suite: ['lfs']
)

memcheck_check = find_program('memcheck-check.sh')
test(
  'memcheck-check', memcheck_check,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()],
  suite: ['valgrind']
)

memcheck_sum = find_program('memcheck-sum.sh')
test(
  'memcheck-sum', memcheck_sum,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()],
  suite: ['valgrind']
)

stdin_cli_dash_sum = find_program('stdin-cli-dash-sum.sh')
test(
  'stdin-cli-dash-sum', stdin_cli_dash_sum,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()]
)

stdin_cli_empty_check = find_program('stdin-cli-empty-check.sh')
test(
  'stdin-cli-empty-check', stdin_cli_empty_check,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()]
)

stdin_cli_empty_sum = find_program('stdin-cli-empty-sum.sh')
test(
  'stdin-cli-empty-sum', stdin_cli_empty_sum,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()]
)

successful_workflow = find_program('successful-workflow.sh')
test(
  'successful-workflow', successful_workflow,
  env: ['MESON_SOURCE_ROOT=' + meson.source_root()]
)
