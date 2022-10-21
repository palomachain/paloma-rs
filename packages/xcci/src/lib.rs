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
//!         job_id: String,
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
//!         job_id,
//!         payload,
//!     } = msg;
//!     Ok(
//!         Response::new().add_message(CosmosMsg::Custom(ExecutePalomaJob {
//!             job_id,
//!             payload,
//!         })),
//!     )
//! }
//! ```

#![deny(missing_docs)]

use cosmwasm_std::{Binary, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A struct implementing `CustomMsg` to be passed as a response message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ExecutePalomaJob {
    /// Metadata of the foreign contract we wish to call.
    pub job_id: String,
    /// Payload for the call, encoded appropriately for the target chain and contract.
    pub payload: Binary,
}

impl CustomMsg for ExecutePalomaJob {}
