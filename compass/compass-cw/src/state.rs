use crate::msg::{Valset, ValsetId};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const VALSET: Item<Valset> = Item::new("valset");
pub const VALSET_ID: Item<ValsetId> = Item::new("valset_id");

pub const SMART_CONTRACT_ID: Item<Addr> = Item::new("smart_contract_id");

pub const MESSAGE_ID_USED: Map<Vec<u8>, ()> = Map::new("message_id_used");
