use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ADMIN: Item<Addr> = Item::new("admin");
pub const DELEGATE_ADDRESS: Item<Addr> = Item::new("delegate_address");
pub const DENOM: Item<String> = Item::new("denom");

pub const REWARDS: Map<Addr, u128> = Map::new("rewards");
pub const CLAIMED_REWARDS: Map<Addr, u128> = Map::new("claimed_rewards");
pub const SUBMITTED: Map<String, ()> = Map::new("submitted");

pub const TOTAL_REGISTERED: Item<u128> = Item::new("total_registered");
pub const TOTAL_CLAIMED: Item<u128> = Item::new("total_claimed");
