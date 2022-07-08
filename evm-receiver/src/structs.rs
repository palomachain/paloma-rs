use cosmwasm_std::ContractInfo;
use ethabi::Address;
use serde::{Deserialize, Serialize};

pub struct Provider {
    pub governance_chain_id: u16,
    pub governance_contract: ContractInfo,
}

#[derive(Serialize, Deserialize)]
pub struct GuardianSet {
    pub keys: Vec<Address>,
    pub expiration_time: u32,
}

pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub v: u8,
    pub guardian_index: u8,
}

pub struct VM {
    pub version: u8,
    pub timestamp: u32,
    pub nonce: u32,
    pub emitter_chain_id: u16,
    pub emitter_address: [u8; 32],
    pub sequence: u64,
    pub consistency_level: u8,
    pub payload: Vec<u8>,

    pub guardian_set_index: u32,
    pub signatures: Vec<Signature>,

    pub hash: [u8; 32],
}
