use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const ADMIN: Item<Addr> = Item::new("admin");
pub const BANK: Item<Uint128> = Item::new("bank");
pub const DENOM: Item<String> = Item::new("denom");

pub const REGISTER: Map<Addr, Uint128> = Map::new("register");
