use cosmwasm_std::{Addr, Binary, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
pub struct ValsetId(Uint256);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Valset {
    pub valset_id: ValsetId,
    pub validators: Vec<Binary>,
    pub powers: Vec<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Signature(pub Vec<u8>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Consensus {
    /// Signatures must be in the same order as the validator array in `valset`
    pub signatures: Vec<Option<Signature>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub smart_contract_id: Addr,
    pub valset: Valset,
}

pub type MessageId = Uint256;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExecuteMsg {
    pub consensus: Consensus,
    pub payload: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecutePayload {
    UpdateValset(Valset, Addr),
    SubmitLogicCall(LogicCallArgs, MessageId, Addr, u64),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LogicCallArgs {
    pub contract_address: Addr,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    SmartContractId,
    ValsetId,
}
