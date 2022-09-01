use cosmwasm_std::{Addr, CustomMsg};
use ethabi::Bytes;
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use schemars::schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub target_contract_info: TargetContractInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    GetDeposit {token_id: u128, lower_tick: i32, depositor: String, deadline: u64},
    PutWithdraw {},
    GetWithdraw {token_ids: Vec<u128>},
    PutCancel {},
    GetCancel {token_ids: Vec<u128>},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    DepositList {},
    WithdrawableList {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenIdList {
    pub list: Vec<u128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TargetContractInfo {
    pub chain_id: String,
    pub compass_id: String,
    pub contract_address: String,
    pub smart_contract_abi: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MultipleIdMsg {
    pub target_contract_info: TargetContractInfo,
    pub method: String,
    pub token_ids: Vec<u128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SingleIdMsg {
    pub target_contract_info: TargetContractInfo,
    pub method: String,
    pub token_id: u128,
}

impl CustomMsg for MultipleIdMsg {}
impl CustomMsg for SingleIdMsg {}
