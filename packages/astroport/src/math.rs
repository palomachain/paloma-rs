use cosmwasm_std::{Decimal, Decimal256, Fraction, Uint128};
use std::convert::TryInto;

/// Convert a `Decimal256` into a `Decimal`
#[inline]
pub fn to_decimal(d: Decimal256) -> Decimal {
    let numerator: Uint128 = d.numerator().try_into().unwrap();
    Decimal::from_atomics(numerator, d.decimal_places()).unwrap()
}

/// Convert a `Decimal` into a `Decimal256`.
#[inline]
pub fn to_decimal256(d: Decimal) -> Decimal256 {
    Decimal256::from_atomics(d.numerator(), d.decimal_places())
        .expect("the range of Decimal256 strictly exceeds Decimal")
}
