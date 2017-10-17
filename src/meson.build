sources = [
  'archive-sum.c',
  'archive-sum.h',
  'check.c',
  'sum.c',
  'util.c',
  'util.h',
]

deps = [libarchive, libcrypto]

executable(
  'archive-sum',
  sources: sources,
  dependencies: deps,
  include_directories: configuration_inc,
  install: true
)
