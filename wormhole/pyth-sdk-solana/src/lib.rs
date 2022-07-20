//! A Rust library for consuming price feeds from the [pyth.network](https://pyth.network/) oracle on the Solana network.
//!
//! Please see the [crates.io page](https://crates.io/crates/pyth-sdk-solana/) for documentation and example usage.

pub use self::error::PythError;

mod error;
pub mod state;

/// Maximum valid slot period before price is considered to be stale.
pub const VALID_SLOT_PERIOD: u64 = 25;
