use cosmwasm_std::CustomMsg;
use pyth_sdk::{PriceFeed, PriceIdentifier};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub target_contract_info: TargetContractInfo,
    pub price_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    GetDeposit {
        token_id: u128,
        lower_tick: i32,
        deadline: u64,
    },
    PutWithdraw {},
    GetWithdraw {
        token_ids: Vec<u128>,
    },
    PutCancel {},
    GetCancel {
        token_ids: Vec<u128>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    DepositList {},
    WithdrawableList {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PythBridgeQueryMsg {
    PriceFeed { id: PriceIdentifier },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenIdList {
    pub list: Vec<u128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TargetContractInfo {
    pub method: String,
    pub chain_id: String,
    pub compass_id: String,
    pub contract_address: String,
    pub smart_contract_abi: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponseMsg {
    pub target_contract_info: TargetContractInfo,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PriceFeedResponse {
    /// Pyth Price Feed
    pub price_feed: PriceFeed,
}

impl CustomMsg for CustomResponseMsg {}
