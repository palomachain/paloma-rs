use crate::msg::{CustomResponseMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::TARGET_CONTRACT_INFO;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

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
    TARGET_CONTRACT_INFO.save(deps.storage, &msg.target_contract_info)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<CustomResponseMsg>> {
    match msg {
        ExecuteMsg::LayEgg { eth_address } => lay_egg(deps, env, info, eth_address),
    }
}

fn lay_egg(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    eth_address: String,
) -> StdResult<Response<CustomResponseMsg>> {
    let target_contract_info = TARGET_CONTRACT_INFO.load(deps.storage)?;
    let eth_address = eth_address.strip_prefix("0x").unwrap();
    assert_eq!(eth_address.len(), 40);
    let eth_address = hex::decode(eth_address).unwrap();
    let eth_address = ethabi::ethereum_types::Address::from_slice(&eth_address);
    let eth_address = ethabi::Token::Address(eth_address);
    let eth_address = ethabi::encode(&[eth_address]);
    Ok(
        Response::new().add_message(CosmosMsg::Custom(CustomResponseMsg {
            target_contract_info,
            paloma_address: info.sender,
            eth_address,
        })),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&())
}
