# Astroport Core

[![codecov](https://codecov.io/gh/astroport-fi/astroport-core/branch/main/graph/badge.svg?token=ROOLZTGZMM)](https://codecov.io/gh/astroport-fi/astroport-core)

Multi pool type automated market-maker (AMM) protocol powered by smart contracts on the [Paloma](https://github.com/palomachain/paloma) blockchain.

## Contracts diagram

```mermaid
classDiagram  
Generator --> Vesting : Claims Rewards
ASTRO Token --> Vesting : Creates schedules for Generator
ASTRO Token --> Periphery contracts
ASTRO Token --> Staking : Locks
Staking --> XASTRO Token : Mints

Maker --> Staking : 2. Transfers ASTRO collected from fees
Maker --> Pools : 1. Collect fees from pools
Maker --> Factory : 1. Swap all fees to ASTRO
Factory --> Pools : Manages Pools
Router --> Factory : Multi-hop swaps

class Periphery contracts {
    Airdrop
    Lockdrop
    Auction
}

class Pools {
    ASTRO-UST
    STABLE
    XYK
}
```

## General Contracts

| Name                                                       | Description                                  |
| ---------------------------------------------------------- | -------------------------------------------- |
| [`factory`](contracts/factory)                             | Pool creation factory                        |
| [`pair`](contracts/pair)                                   | Pair with x*y=k curve                        |
| [`pair_stable`](contracts/pair_stable)                     | Pair with stableswap invariant curve         |
| [`token`](contracts/token)                                 | CW20 (ERC20 equivalent) token implementation |
| [`router`](contracts/router)                               | Multi-hop trade router                       |
| [`oracle`](contracts/periphery/oracle)                     | TWAP oracles for x*y=k pool types            |
| [`whitelist`](contracts/whitelist)                         | CW1 whitelist contract                       |

## Tokenomics Contracts

Tokenomics related smart contracts are hosted on ../contracts/tokenomics.

| Name                                                       | Description                                      |
| ---------------------------------------------------------- | ------------------------------------------------ |
| [`generator`](contracts/tokenomics/generator)                                   | Rewards generator for liquidity providers        |
| [`generator_proxy_to_mirror`](contracts/tokenomics/generator_proxy_to_mirror)   | Rewards generator proxy for liquidity providers  |
| [`maker`](contracts/tokenomics/maker)                                           | Fee collector and swapper                        |
| [`staking`](contracts/tokenomics/staking)                                       | xASTRO staking contract                          |
| [`vesting`](contracts/tokenomics/vesting)                                       | ASTRO distributor for generator rewards          |
| [`xastro_token`](contracts/tokenomics/xastro_token)                             | xASTRO token contract                            |

## Building Contracts

You will need Rust 1.58.1+ with wasm32-unknown-unknown target installed.

You can run unit tests for each contract directory via:

```
cargo test
```

#### For a production-ready (compressed) build:
Run the following from the repository root

```
./scripts/build_release.sh
```

The optimized contracts are generated in the artifacts/ directory.

#### You can compile each contract:
Go to contract directory and run

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/astroport_token.wasm .
ls -l astroport_token.wasm
sha256sum astroport_token.wasm
```

## Docs

Docs can be generated using `cargo doc --no-deps`

## Bug Bounty

The contracts in this repo are included in a [bug bounty program](https://www.immunefi.com/bounty/astroport).
