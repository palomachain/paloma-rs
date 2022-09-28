#![allow(clippy::derive_partial_eq_without_eq)]

pub mod contract;
pub mod msg;

use crate::msg::Valset;
use anchor_lang::prelude::*;
use std::collections::HashSet;

// TODO: This value needs to change (after deploy?).
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[account]
pub struct Compass<'a> {
    pub valset: Valset,

    pub program_id: Program<'a, X>,

    pub message_id_used: HashSet<[u8; 32]>,
}
