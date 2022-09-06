#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(deprecated)]

pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;
