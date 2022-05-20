# Paloma Bindings for CosmWasm

This crate provides Paloma-specific bindings to enable your CosmWasm smart contracts to interact
with the Paloma blockchain by exposing messages and queriers that can be emitted and used from within your contract.

## Contents

Currently, the Paloma bindings include:

- Query support for:
  - Market
    - swap rate between 2 currencies at market price
  - Treasury
    - current tax cap for a denomination
    - current tax rate 
  - Oracle
    - exchange rates for the given base_denom / quote_denoms

- Messages
  - `MsgSwap`
  - `MsgSwapSend`

## Usage

### Querying

In order to use the query functions enabled by the bindings,
create a `PalomaQuerier` instance within your contract logic -- in either `init()`, `handle()`, or `query()` entrypoints.
You can access all enabled queries through this object.

```rust
// src/contract.rs
use cosmwasm_std::Coin;
use paloma_cosmwasm::{ PalomaQuerier, SwapResponse, TaxRateResponse, TaxCapResponse, ExchangeRatesResponse };

...

// handler
pub fn try_something<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    offer: &Coin
) -> StdResult<HandleResponse> {
    let querier = PalomaQuerier::new(&deps.querier);
    let swap_rate: SwapResponse = querier.query_swap(offer.clone(), "uusd")?;
    let tax_cap: TaxCapResponse = querier.query_tax_cap("usdr")?;
    let tax_rate: TaxRateResponse = querier.query_tax_rate()?;
    let exchange_rates: ExchangeRatesResponse = querier.query_exchange_rates("uusd", vec!["uluna", "ukrw"])?;
    ...
}
```

## Creating Messages

The Paloma bindings do not cover messages that have already been implemented by the CosmWasm team,
such as staking-related messages and fundamental ones like `MsgSend`.

You may want your contract to perform operations such as `MsgSwap` and `MsgSwapSend` at the end of its execution.
To do this, create a message using the predefined functions:

- `create_swap_msg`
- `create_swap_send_msg`

And add it to the vector of `messages` in your `HandleResponse` before you return.

```rust
use cosmwasm_std::CosmosMsg;
use paloma_cosmwasm::{create_swap_msg, PalomaMsgWrapper};

...

pub fn try_something<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    offer: &Coin
) -> StdResult<HandleResponse<PalomaMsgWrapper>> {
    ...

    let msg: CosmosMsg<PalomaMsgWrapper> = create_swap_msg(contract_addr, offer_coin, ask_denom);
    let res = HandleResponse {
        messages: vec![msg],
        log: vec![],
        data: None
    };
    Ok(res)
}
```
