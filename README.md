Whetstone OS
============

[![Build Status](https://travis-ci.org/orumin/Whetstone_OS.svg?branch=master)](https://travis-ci.org/orumin/Whetstone_OS)

What's this?
------------

Hobby OS written by The Rust Programming Language.
This OS is expected x86_64 and UEFI PC.

How to build
------------

- First, prepare GNU binutils, its target for x86_64-efi-pe
- Second, you have to use Rust nightly compiler.

```sh
$ rustup install nightly
$ rustup default nightly
$ cargo install xargo
$ export PATH="$HOME/.cargo/bin:$PATH"
```

Then, only you run `make` on root directory.
