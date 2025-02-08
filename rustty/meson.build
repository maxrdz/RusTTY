# ----- Cargo Build ----- #

crate_cargo_opts = [ '--package', base_binary ]
feature_flags = []

# Convert FF list to argument string for --features option
cargo_ff_arg = ''
first_flag = false
foreach ff : feature_flags
  if first_flag == false
    cargo_ff_arg += ff
    first_flag = true
  else
    cargo_ff_arg += ',' + ff
  endif
endforeach

if feature_flags.length() > 0
  crate_cargo_opts += [ '--no-default-features', '--features', cargo_ff_arg ]
else
  message('Building with default feature flags.')
endif

crate_build = custom_target(
  'program-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'program-cargo-build', # see `program-wrapper` target
  console: true,
  env: cargo_env,
  command: [
    cargo_bin, 'build', cargo_opts, crate_cargo_opts,
  ]
)

# Replace below with `--artifact-dir` cargo option once stable.
# Issue: https://github.com/rust-lang/cargo/issues/6790
custom_target(
  'rustty',
  build_by_default: true,
  build_always_stale: true,
  output: binary,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  env: cargo_env,
  command: [
    'cp', target_dir / base_binary, '@BUILD_ROOT@' / base_binary / binary,
  ],
  depends: crate_build,
)
