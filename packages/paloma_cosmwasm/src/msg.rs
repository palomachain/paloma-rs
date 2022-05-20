use cosmwasm_std::{Coin, CosmosMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::PalomaRoute;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// An override of [`CosmosMsg::Custom`] to show this contract can be extended.
pub struct PalomaMsgWrapper {
    pub route: PalomaRoute,
    pub msg_data: PalomaMsg,
}

// define trait bound
impl cosmwasm_std::CustomMsg for PalomaMsgWrapper {}

// this is a helper to be able to return these as CosmosMsg easier
impl From<PalomaMsgWrapper> for CosmosMsg<PalomaMsgWrapper> {
    fn from(original: PalomaMsgWrapper) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PalomaMsg {
    Swap {
        offer_coin: Coin,
        ask_denom: String,
    },
    SwapSend {
        to_address: String,
        offer_coin: Coin,
        ask_denom: String,
    },
}

// create_swap_msg returns wrapped swap msg
pub fn create_swap_msg(offer_coin: Coin, ask_denom: String) -> CosmosMsg<PalomaMsgWrapper> {
    PalomaMsgWrapper {
        route: PalomaRoute::Market,
        msg_data: PalomaMsg::Swap {
            offer_coin,
            ask_denom,
        },
    }
    .into()
}

// create_swap_send_msg returns wrapped swap send msg
pub fn create_swap_send_msg(
    to_address: String,
    offer_coin: Coin,
    ask_denom: String,
) -> CosmosMsg<PalomaMsgWrapper> {
    PalomaMsgWrapper {
        route: PalomaRoute::Market,
        msg_data: PalomaMsg::SwapSend {
            to_address,
            offer_coin,
            ask_denom,
        },
    }
    .into()
}
