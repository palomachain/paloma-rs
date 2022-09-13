#![allow(clippy::derive_partial_eq_without_eq)]

pub mod contract;
pub mod msg;
pub mod state;
pub mod token_address;

#[cfg(test)]
mod testing;

// Chain ID of Paloma
pub const CHAIN_ID: u16 = 48;
