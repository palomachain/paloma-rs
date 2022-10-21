use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use std::collections::HashSet;

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const JOB_ID: Item<String> = Item::new("job_id");

pub const ENTRANTS: Map<Addr, String> = Map::new("entrants");
pub const PALOMA_WINNERS: Item<HashSet<Addr>> = Item::new("paloma_winners");
pub const ETH_WINNERS: Item<HashSet<String>> = Item::new("eth_winners");
