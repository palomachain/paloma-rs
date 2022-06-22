use crate::msg::TargetContractInfo;
use cw_storage_plus::Item;

pub const TARGET_CONTRACT_INFO: Item<TargetContractInfo> = Item::new("target_contract_info");
