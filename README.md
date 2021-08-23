# Scrypto

[![CI](https://github.com/radixdlt/radixdlt-scrypto/actions/workflows/ci.yml/badge.svg)](https://github.com/radixdlt/radixdlt-scrypto/actions/workflows/ci.yml)

Language for building DeFi apps on Radix.

## Installation

1. Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Install WebAssembly toolchain
```
rustup target add wasm32-unknown-unknown
```
3. Install Radix Engine simulator
```
git clone git@github.com:radixdlt/radixdlt-scrypto.git
cd radixdlt-scrypto/simulator
cargo install --path .
```

## Getting Started

1. Create a new package by copying one from [the examples](./examples), and then build
```
cargo build --target wasm32-unknown-unknown --release
```
2. To create a new account (the first account will be used as the default account), run
```
rev2 new-account
```
3. To publish your package, run
```
rev2 publish-package /path/to/your/package
```
4. To invoke a blueprint function, run
```
rev2 invoke-blueprint <package_address> <blueprint> <function> <args>...
```
5. To invoke a component method, run
```
rev2 invoke-component <component_address> <method> <args>...
```
6. For instructions on other commands, run
```
rev2 help
```

## Project Layout

![](./assets/crate-dependencies.svg)

- `sbor`: Scrypto Binary Object Representation (SBOR), the data format for Scrypto.
- `sbor-derive`: SBOR derives for Rust `struct` and `enum`.
- `scrypto`: Scrypto standard library.
- `scrypto-abi`: Scrypto JSON-exportable blueprint ABI.
- `scrypto-types`: Scrypto primitive types.
- `scrypto-derive`: Derives for creating and importing Scrypto blueprints.
- `radix-engine`: Radix Engine, the Scrypto execution layer.
