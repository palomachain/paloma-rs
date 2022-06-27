use crate::msg::{CustomResponseMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{ADMIN, ENTRANTS, ETH_WINNERS, PALOMA_WINNERS, TARGET_CONTRACT_INFO};
use cosmwasm_std::{
    coin, ensure_eq, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdResult,
};
use eyre::{bail, ensure, eyre, Result};
use rand::seq::IteratorRandom;
use rand::SeedableRng;
use std::collections::HashSet;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

pub(crate) const ENTRANCE_FEE: u128 = 1_000_000;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ADMIN.save(deps.storage, &info.sender)?;
    TARGET_CONTRACT_INFO.save(deps.storage, &msg.target_contract_info)?;
    PALOMA_WINNERS.save(deps.storage, &HashSet::new())?;
    ETH_WINNERS.save(deps.storage, &HashSet::new())?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<CustomResponseMsg>> {
    match msg {
        ExecuteMsg::LayEgg { eth_address } => lay_egg(deps, env, info, eth_address),
        ExecuteMsg::PickWinner {} => pick_winner(deps, env, info),
    }
}

fn lay_egg(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    eth_address: String,
) -> Result<Response<CustomResponseMsg>> {
    // We need a valid looking eth address
    assert_eq!(
        hex::decode(eth_address.strip_prefix("0x").unwrap())
            .unwrap()
            .len(),
        20
    );

    let entry_fee = coin(ENTRANCE_FEE, "ugrain");
    // Not depositing anywhere, just dropping on the floor.
    ensure!(
        info.funds == [entry_fee],
        "Entry fee is 1 grain, please supply funds."
    );

    let paloma_winners = PALOMA_WINNERS.load(deps.storage)?;
    if paloma_winners.contains(&info.sender) {
        bail!("This address has already won an egg.");
    }
    let eth_winners = ETH_WINNERS.load(deps.storage)?;
    if eth_winners.contains(&eth_address) {
        bail!("This ETH address has already won an egg.");
    }

    ENTRANTS.save(deps.storage, info.sender, &eth_address)?;
    Ok(Response::new())
}

fn pick_winner(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response<CustomResponseMsg>> {
    ensure_eq!(info.sender, ADMIN.load(deps.storage)?, eyre!("forbidden"));

    let mut paloma_winners = PALOMA_WINNERS.load(deps.storage)?;
    let mut eth_winners = ETH_WINNERS.load(deps.storage)?;
    let (paloma_address, eth_address_str) = ENTRANTS
        .range(deps.storage, None, None, Order::Ascending)
        .filter(|e| match e {
            Ok((paloma_address, eth_address)) => {
                !(paloma_winners.contains(paloma_address) || eth_winners.contains(eth_address))
            }
            Err(_) => false,
        })
        .choose(&mut rand::rngs::StdRng::seed_from_u64(
            env.block.time.nanos(),
        ))
        .unwrap()?;
    paloma_winners.insert(paloma_address.clone());
    eth_winners.insert(eth_address_str.clone());
    PALOMA_WINNERS.save(deps.storage, &paloma_winners)?;
    ETH_WINNERS.save(deps.storage, &eth_winners)?;

    let target_contract_info = TARGET_CONTRACT_INFO.load(deps.storage)?;
    let eth_address = hex::decode(eth_address_str.strip_prefix("0x").unwrap()).unwrap();
    let eth_address = ethabi::ethereum_types::Address::from_slice(&eth_address);
    let eth_address = ethabi::Token::Address(eth_address);
    let eth_address = ethabi::encode(&[eth_address]);
    Ok(Response::new()
        .add_message(CosmosMsg::Custom(CustomResponseMsg {
            target_contract_info,
            paloma_address: paloma_address.clone(),
            eth_address,
        }))
        .add_attribute("winning_paloma_address", &paloma_address)
        .add_attribute("winning_eth_address", &eth_address_str))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&())
}
