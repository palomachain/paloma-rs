use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ADMIN: Item<Addr> = Item::new("admin");
pub const DELEGATE_ADDRESS: Item<Addr> = Item::new("delegate_address");
pub const DENOM: Item<String> = Item::new("denom");

pub const REWARDS: Map<Addr, u128> = Map::new("rewards");
pub const TOTAL_CLAIMED: Map<Addr, u128> = Map::new("total_claimed");
pub const SUBMITTED: Map<String, ()> = Map::new("submitted");
