# Pebble
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)
[![Gitter chat](https://badges.gitter.im/gitterHQ/gitter.png)](https://gitter.im/pebble-os/Lobby)

**Pebble is still early in development.**

Pebble is a microkernel and userspace written in Rust, with a focus on safety and simplicity. It is designed to be
simple to understand, extend, and develop for. Pebble does not aim for POSIX compliance. The best way to learn
about Pebble is to read [the book](https://isaacwoods.github.io/pebble/book/).
[The website](https://isaacwoods.github.io/pebble) also hosts some other useful resources.

## Building and running
**Operating systems tend to be complex to build and run. We've tried to make this as simple as we can, but if you
encounter problems or have suggestions to make it easier, feel free to file an issue :)**

### Getting the source
Firstly, clone the repository and fetch the submodules:
```
git clone https://github.com/IsaacWoods/pebble.git
git submodule update --init --recursive
```

### Things you'll need
- A nightly Rust toolchain
- The `rust-src` component (install with `rustup component add rust-src`)
- A working QEMU installation (one that provides `qemu-system-{arch}`)

To compile userspace programs, you'll need to build a custom Rust toolchain (hopefully just for now):
- Clone [`IsaacWoods/rust`](https://github.com/IsaacWoods/rust/tree/pebble) and checkout the `pebble` branch
- (Optional) rebase against `rust-lang/rust` to get the latest chages
- Copy `isaacs_config.toml` to `config.toml` (or use your own)
- Run `./x.py build -i library/std` to build a stage-1 compiler and `libstd`
- Create a toolchain with `rustup toolchain link pebble build/{host triple}/stage1` (e.g. `rustup toolchain link pebble build/x86_64-unknown-linux-gnu/stage1`)

### Using `butler` to build and run projects
This repository includes a build tool, `butler`, to simplify building and running Pebble. It is configured as a
Cargo alias, `cargo bu`, to make it easy to access.

Running `cargo bu` will build a standard Pebble distribution and run it in QEMU. See `cargo bu -- --help` for how
to do more elaborate things with it, including a list of projects.

## Contributing
You are very welcome to contribute to Pebble! Have a look at the issue tracker, or come hang out in the Gitter room
to find something to work on.

Any contribution submitted for inclusion in Pebble by you shall be licensed according to the MPL-2.0, without
additional terms or conditions.
