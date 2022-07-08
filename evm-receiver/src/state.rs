use crate::structs::{GuardianSet, Provider};
use cosmwasm_std::{Addr, Env};
use cw_storage_plus::{Item, Map};
use ethabi::Address;

/* TODO
contract Events {
    event LogGuardianSetChanged(
        uint32 oldGuardianIndex,
        uint32 newGuardianIndex
    );

    event LogMessagePublished(
        address emitter_address,
        uint32 nonce,
        bytes payload
    );
}
 */

pub const PROVIDER: Item<Provider> = Item::new("provider");

/// Contract deployer
pub const OWNER: Item<Addr> = Item::new("owner");

/// Mapping of guardian_set_index => guardian set
pub const GUARDIAN_SETS: Map<u32, GuardianSet> = Map::new("guardian_sets");

/// Current active guardian set index
pub const GUARDIAN_SET_INDEX: Item<u32> = Item::new("guardian_set_index");

/// Period for which a guardian set stays active after it has been replaced
const GUARDIAN_SET_EXPIRY: Item<u32> = Item::new("guardian_set_expiry");

/// Mapping of consumed governance actions
pub const CONSUMED_GOVERNANCE_ACTIONS: Map<[u8; 32], ()> = Map::new("consumed_governance_actions");

/// Mapping of initialized implementations
pub const INITIALIZED_IMPLEMENTATIONS: Map<Address, ()> = Map::new("initialized_implementations");
