use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map};
use crate::msg::TargetContractInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub lower_tick: i32,
    pub deadline: u64
}

pub const TARGET_CONTRACT_INFO: Item<TargetContractInfo> = Item::new("target_contract_info");

pub const PRICE: Item<f32> = Item::new("price");

pub const DEPOSIT: Map<u128, Deposit> = Map::new("deposit");
