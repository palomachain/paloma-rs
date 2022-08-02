use arrayvec::ArrayVec;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use std::mem::size_of;

const MAX_PAYLOAD: usize = 20480;

struct Item<T: BorshSerialize + BorshDeserialize, const O: usize>;

impl<T: BorshSerialize + BorshDeserialize, const Off: usize> Item<T, Off> {
    pub fn store(account: &AccountInfo, input: &T) {
        let mut buf: ArrayVec<u8, MAX_PAYLOAD> = ArrayVec::new();
        input.serialize(&mut buf).unwrap();
        let mut account_data = account.data.borrow_mut();
        account_data[Off..Off + buf.len()].copy_from_slice(&buf);
    }

    pub fn load(account: &AccountInfo) -> T {
        let mut account_data = account.data.borrow_mut();
        T::deserialize(&mut account_data[Off..Off + buf.len()])
    }
}

/// Unique identifier for this turnstone instance.
pub const TURNSTONE_ID: Item<[u8; 32], 0> = Item;

pub const LAST_CHECKPOINT: Item<[u8; 32], 32> = Item;
//pub const MESSAGE_ID_USED: Map<Uint256, ()> = Map::new("message_id_used");
