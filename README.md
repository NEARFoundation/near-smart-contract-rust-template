# NEAR Smart Contract Rust Template

Project structure for writing smart contracts for NEAR in Rust.

## Required Software

- [Rust & Cargo](https://www.rust-lang.org/tools/install)
  - With the WASM target installed: `rustup target add wasm32-unknown-unknown`
  - [cargo-make](https://crates.io/crates/cargo-make): `cargo install cargo-make`

## Usage

### Scripts

#### `cargo make clean`

Removes the `target` and `neardev` directories.

#### `cargo make test`

Runs unit tests using the default target. (Note: behavior may differ from simply running `cargo test` depending on the target specified in `.cargo/config.toml`.)

#### `cargo make build`

Compiles the smart contract to a WebAssembly binary. The binary path is `./target/wasm32-unknown-unknown/release/<package>.wasm`.

#### `cargo make optimize`

Cleans up and optimizes the most recently-built WASM binary. The optimized binary path is `./deploy/<name>.wasm`.

**Note**: In order to optimize the WASM binary, you must have [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools) installed:

```txt
cargo install wasm-tools
```

`wasm-tools` is also available on Homebrew:

```txt
brew install wasm-tools
```

## Authors

- Jacob Lindahl <jacob.lindahl@near.org> [@sudo_build](https://twitter.com/sudo_build)
