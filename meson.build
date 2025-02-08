project('rustty', 'rust',
          version: '0.1.0',
    meson_version: '>= 1.1.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

rootdir = ''
prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')
datadir = prefix / get_option('datadir')

# build dependencies
cargo_bin = find_program('cargo', required: true)
find_program('ninja', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)
appstream_cli = find_program('appstreamcli', required: false)

# app global variables
app_name = 'RusTTY'
base_id = 'org.rustty.RusTTY'
base_binary = 'rustty'
git_url = 'https://gitlab.com/rustty/rustty'

# Cargo build options and environment variables
cargo_opts = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opts += [ '--target-dir', meson.project_build_root()  / 'target' ]
cargo_opts += [ '--color=always' ]

cargo_env = environment()
cargo_env.set('CARGO_HOME', meson.project_build_root() / 'cargo-home')

if get_option('profile') == 'release'
  cargo_opts += [ '--release' ]
  rust_target = 'release'

  app_id = base_id
  binary = base_binary

  message('Building for Release.')
else
  rust_target = 'debug'

  app_id = '@0@.Debug'.format(base_id)
  binary = '@0@-debug'.format(base_binary)

  message('Building for Debug.')
endif

target_dir = meson.project_build_root() / 'target' / rust_target

# ---------- Cross Compilation ---------- #

if get_option('target') != ''
  cargo_opts += [ '--target', get_option('target') ]

  target_dir = 'target' / get_option('target') / rust_target

  # For cross compiling, we use the cross-rs
  # project for building our application via Docker.
  # This makes the cross compilation process simpler,
  # and requires less setup on the developer's machine.

  docker_bin = find_program(['docker', 'podman']) # podman also works
  cargo_bin = find_program('cross')

  if get_option('cross-verbose')
    cargo_opts += [ '-v' ]
  endif

  # Emulate arm64 instead of using native cross compilation tools
  # (why emulate? because cross compilation is such a pain)
  run_command(
    docker_bin, 'run', '--privileged',
    '--rm', 'tonistiigi/binfmt', '--install', 'arm64',
    check: true,
  )
endif

# Install Directories
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'i18n'

base_dir = join_paths(rootdir, prefix)

bin_src = meson.project_build_root() / binary
bin_dst = base_dir / 'bin' / binary

subdir('rustty')

clippy = custom_target(
  'clippy',
  env: cargo_env,
  output: 'clippy',
  command: [
    cargo_bin, 'clippy', '--all', cargo_opts,
  ],
)

run_target(
  'linting',
  env: cargo_env,
  command: [
    cargo_bin, 'fmt', '--all', '--', '--color=always', '--check',
  ],
  depends: clippy,
)

cargo_deny = custom_target(
  'cargo-deny',
  env: cargo_env,
  output: 'cargo-deny',
  command: [
    cargo_bin, 'install', 'cargo-deny', '--locked',
  ],
)

run_target(
  'deny',
  env: cargo_env,
  command: [
    cargo_bin, 'deny', '--manifest-path',
    meson.project_source_root() / 'Cargo.toml', 'check',
  ],
  depends: cargo_deny
)

run_target(
  'cargo-clean',
  env: cargo_env,
  command: [
    cargo_bin, 'clean',
  ],
)

message('Setting up Git hooks..')
run_command('cp', '-f', 'build-aux/git/pre-commit.hook', '.git/hooks/pre-commit', check: false)
run_command('cp', '-f', 'build-aux/git/pre-push.hook', '.git/hooks/pre-push', check: false)

meson.add_dist_script(
  'build-aux/meson/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
  meson.project_source_root()
)
