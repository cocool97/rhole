# rhole: Rust local DNS adblocker

[![Latest version](https://img.shields.io/crates/v/rhole.svg)](https://crates.io/crates/rhole)
[![dependency status](https://deps.rs/repo/github/cocool97/rhole/status.svg)](https://deps.rs/repo/github/cocool97/rhole)
[![codecov](https://codecov.io/gh/cocool97/rhole/branch/master/graph/badge.svg?token=2PMZ6D9E5M)](https://codecov.io/gh/cocool97/rhole)

Highly configurable Rust local DNS adblocker.

## Main advantages

* Highly configurable
  * Use of many kinds of blocking lists
  * Remote DNS proxy server configuration
* Easy to setup
  * Can be cross-compiled to various targets
  * Default configuration file
  * RPM package build

## How does it work ?

`rhole` reads blacklist entries from various remote sources, inserts them in a [sled](https://docs.rs/sled/latest/sled/) database and checks against it for every DNS entries asked.

## Compilation

* Fedora

```bash
# Adds support for arm-gnueabihf toolchain
sudo dnf copr enable lantw44/arm-linux-gnueabihf-toolchain
sudo dnf install arm-linux-gnueabihf-{binutils,gcc,glibc}

# armv7-musl
CC=arm-linux-gnueabihf-gcc cargo build --release --target armv7-unknown-linux-musleabihf

# armv7
cargo build --release --target armv7-unknown-linux-gnueabihf
```

* Ubuntu

```bash
TODO
```

## Work to do

This is an in-progress work, many things must still be achieved to reach version `1.0.0`:

* Web monitoring interface
* Code audit to improve performances + possible mistakes
* Regularly update input sources
* Making everything `async`
* Increase DNS record TTL for blocked addresses
* Log requests per equipments
