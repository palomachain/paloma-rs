#![allow(clippy::derive_partial_eq_without_eq)]

extern crate core;
extern crate cosmwasm_std;

pub mod contract;
pub mod error;
pub mod state;
pub mod utils;

#[cfg(test)]
mod testing;
