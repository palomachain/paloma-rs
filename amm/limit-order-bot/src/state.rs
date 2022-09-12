use cosmwasm_std::Uint256;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::TargetContractInfo;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub sqrt_price_x96: Uint256,
    pub deadline: u64,
}

pub const TARGET_CONTRACT_INFO: Item<TargetContractInfo> = Item::new("target_contract_info");

pub const PRICE_CONTRACT: Item<String> = Item::new("price_contract");

pub const DEPOSIT: Map<u128, Deposit> = Map::new("deposit");

pub const ETH_USD: &str = "ca80ba6dc32e08d06f1aa886011eed1d77c77be9eb761cc10d72b7d0a2fd57a6";
