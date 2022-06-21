use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
};
use std::default::Default;

use crate::state::COMPASS_ID;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    COMPASS_ID.save(deps.storage, &msg.compass_id)?;
    Ok(Default::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::LayEgg { eth_address } => lay_egg(deps, env, info, eth_address),
    }
}

fn lay_egg(deps: DepsMut, env: Env, info: MessageInfo, eth_address: String) -> StdResult<Response> {
    let compass_id = COMPASS_ID.load(deps.storage)?;
    Ok(Response::new().add_event(
        Event::new("lay_egg")
            .add_attribute("address", &info.sender)
            .add_attribute("eth_address", eth_address)
            //TODO .add_attribute("payload", "abc")
            //TODO .add_attribute("contract_abi", "xyz")
            .add_attribute("chain_id", env.block.chain_id)
            .add_attribute("compass_id", compass_id),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&())
}
