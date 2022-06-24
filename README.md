# Paloma Rust

Contracts and infrastructure related to the [Paloma](https://github.com/palomachain/paloma) Blockchain.

## Crates

| Name                                   | Description                                   |
|----------------------------------------|-----------------------------------------------|
| [`dex`](dex)                           | Automatic market maker                        |
| [`dex-tokenomics`](dex-tokenomics)     | Tokenomics related smart contracts            |
| [`egg`](egg)                           | Simple contract for a minting contest         |
| [`wormhole`](wormhole/)                | Contracts for communicating with other chains |

## Building

You will need Rust 1.58.1+ with wasm32-unknown-unknown target installed.

You can run unit tests with

```
cargo test
```

#### For a production-ready (compressed) build:
Run the following from the repository root

```
./scripts/build_release.sh
```

The optimized contracts are generated in the artifacts/ directory.

You can compile contracts to wasm with `cargo wasm`.
The compiled wasm can be found at `target/wasm32-unknown-unknown/release/<contract>.wasm`.
