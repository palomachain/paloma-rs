use anchor_lang::prelude::Pubkey;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct MigrateMsg {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct Valset {
    pub valset_id: u128,
    pub validators: Vec<Pubkey>,
    pub powers: Vec<u32>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct Signature(pub Vec<u8>);

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct Consensus {
    /// Signatures must be in the same order as the validator array in `valset`
    pub signatures: Vec<Signature>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct LogicCallArgs {
    /// The arbitrary contract address to external call.
    pub contract_address: Pubkey,
    pub payload: Vec<u8>,
}
