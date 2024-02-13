# Modifications installer

## Installation of essential tools

1. Install the Rust programming language toolchain called `rustup`.
2. Install the latest version of nightly `rust`.
3. Set the nightly `rust` as the default toolchain for this project.
4. (optional) Install NodeJS/Bun with npm/yarn/pnpm to run commands from package.json.

## Setup

1. Create `.env` file.
2. Create inside `.env` a variable called `FOLDER_NAME`, and set one of the allowed values. You can find allowed values in `build.rs` file.

## Commands

Build production ready installer:

```sh
[yarn/pnpm/npm/bun] run build
# or
cargo clean && cargo build --release
```

Build development version of installer without any optimizations:

```sh
[yarn/pnpm/npm/bun] run build-development
# or
cargo clean && cargo build
```

## Where to find builded binary?

Get into `target/[debug/release]`. There should be a `instalator` binary file. Depends on operating system it might be without any extension, or will appear with `.exe` extension.
