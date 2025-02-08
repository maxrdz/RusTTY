# RusTTY

### The TTY shell of the future, written in Rust.

## Building

### Getting the source

```sh
git clone https://gitlab.com/rustty/rustty.git
cd rustty/
```


To build, run the following in the repo root directory:

```sh
meson setup build
meson compile -C build
```

You can append the `-Dprofile=debug` argument to build for debug.

## Installing

To install a build, run:

```sh
meson install -C build
```

## Cross Compiling

PLEASE install cross-rs via:

```sh
cargo install cross --git https://github.com/cross-rs/cross
```

The cross project is in a weird state where it doesn't have much motivation
and/or time to cut a release, so you need to pull from the main branch to
get a lot of bug fixes since the 'latest' release as of December 2024.

Then, you can run:

```sh
meson setup build -Dtarget=aarch64-unknown-linux-gnu
meson compile -C build
```

## Copyright and Software License

Copyright (c) 2025 Max Rodriguez <me@maxrdz.com> 

"RusTTY" can be found at https://gitlab.com/rustty/rustty

"RusTTY" is distributed under the terms of the GNU General Public
License, either version 3.0 or, at your option, any later
version WITHOUT ANY WARRANTY. You can read the full copy of
the software license in the COPYING file.
