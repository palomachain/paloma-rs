//! This crate implements the foreign chain call interface for Paloma.
//! Types here may be used to issue an event which initiates a call on
//! another blockchain. Example:
//!
//! ```rust
//! use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, entry_point};
//! use xcci::{ExecutePalomaJob, TargetContractInfo};
//!
//! pub enum ExecuteMsg {
//!     Call {
//!         target_contract_info: TargetContractInfo,
//!         payload: Binary,
//!     }
//! }
//!
//! #[entry_point]
//! pub fn execute(
//!     _deps: DepsMut,
//!     _env: Env,
//!     _info: MessageInfo,
//!     msg: ExecuteMsg,
//! ) -> Result<Response<ExecutePalomaJob>, StdError> {
//!     let ExecuteMsg::Call {
//!         target_contract_info,
//!         payload,
//!     } = msg;
//!     Ok(
//!         Response::new().add_message(CosmosMsg::Custom(ExecutePalomaJob {
//!             target_contract_info,
//!             payload,
//!         })),
//!     )
//! }
//! ```

#![deny(missing_docs)]

use cosmwasm_std::{Binary, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Metadata necessary to call a specific contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TargetContractInfo {
    /// The chain id of the target chain, e.g. "eth-main".
    pub chain_id: String,
    /// ID of the target chain's compass contract, e.g. "50".
    pub compass_id: String,
    /// The address of the contract to run on the target chain,
    /// e.g. "0xd58Dfd5b39fCe87dD9C434e95428DdB289934179".
    pub contract_address: String,
    /// The json encoded ABI of the contract on the target chain.
    pub smart_contract_abi: String,
}

/// A struct implementing `CustomMsg` to be passed as a response message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ExecutePalomaJob {
    /// Metadata of the foreign contract we wish to call.
    pub target_contract_info: TargetContractInfo,
    /// Payload for the call, encoded appropriately for the target chain and contract.
    pub payload: Binary,
}

impl CustomMsg for ExecutePalomaJob {}
