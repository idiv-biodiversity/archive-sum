project(
  'archive-sum',
  'c',
  version: run_command('git', 'describe', '--always', '--dirty').stdout().strip(),
  license: ['MIT'],
  default_options: ['c_std=gnu11']
)

cdata = configuration_data()
cdata.set_quoted('PACKAGE_NAME', meson.project_name())
cdata.set_quoted('PACKAGE_VERSION', meson.project_version())
configure_file(output: 'config.h', configuration: cdata)
configuration_inc = include_directories('.')

libarchive = dependency('libarchive')
libcrypto = dependency('libcrypto')

subdir('src')
subdir('test')
