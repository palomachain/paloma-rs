use crate::msg::TargetContractInfo;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use std::collections::HashSet;

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const TARGET_CONTRACT_INFO: Item<TargetContractInfo> = Item::new("target_contract_info");

pub const ENTRANTS: Map<Addr, String> = Map::new("entrants");
pub const PALOMA_WINNERS: Item<HashSet<Addr>> = Item::new("paloma_winners");
pub const ETH_WINNERS: Item<HashSet<String>> = Item::new("eth_winners");
