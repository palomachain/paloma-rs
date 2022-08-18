#![allow(clippy::derive_partial_eq_without_eq)]

pub mod contract;
pub mod state;

pub mod error;

mod operations;

#[cfg(test)]
mod testing;
