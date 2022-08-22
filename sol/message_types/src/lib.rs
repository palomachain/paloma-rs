use serde::{Deserialize, Serialize};

/// A serializable version of `ReplicaAccountInfo`
#[derive(Deserialize, Serialize)]
pub struct AccountInfo<'a> {
    pub pubkey: &'a [u8],
    pub lamports: u64,
    pub owner: &'a [u8],
    pub executable: bool,
    pub rent_epoch: u64,
    pub data: &'a [u8],
    pub write_version: u64,
    //pub txn_signature: Option<&'a Signature>,
}
