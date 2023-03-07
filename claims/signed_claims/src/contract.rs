use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{
    ADMIN, CLAIMED_REWARDS, DELEGATE_ADDRESS, DENOM, REWARDS, SUBMITTED, TOTAL_CLAIMED,
    TOTAL_REGISTERED,
};
use cosmwasm_std::{
    coin, coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response,
    StdResult, Uint128,
};
use eyre::{ensure, Result};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response> {
    ADMIN.save(deps.storage, &info.sender)?;
    DELEGATE_ADDRESS.save(deps.storage, &msg.delegate_address)?;
    TOTAL_REGISTERED.save(deps.storage, &0)?;
    TOTAL_CLAIMED.save(deps.storage, &0)?;
    ensure!(
        !info.funds.is_empty(),
        "contract must be instantiated with funds"
    );
    DENOM.save(deps.storage, &info.funds[0].denom)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response> {
    match msg {
        ExecuteMsg::Register { tx_meta } => {
            ensure!(
                info.sender == DELEGATE_ADDRESS.load(deps.storage)?,
                "only the delegate address may register claims"
            );
            ensure!(
                !SUBMITTED.has(deps.storage, tx_meta.tx_hash.clone()),
                "tx already submitted"
            );
            SUBMITTED.save(deps.storage, tx_meta.tx_hash, &())?;
            let reward = REWARDS.update(deps.storage, tx_meta.address.clone(), |reward| {
                StdResult::Ok(reward.unwrap_or(0) + tx_meta.amount.u128())
            })?;
            TOTAL_REGISTERED.update(deps.storage, |total| {
                StdResult::Ok(total + tx_meta.amount.u128())
            })?;
            // TODO: ensure we cannot overcommit.
            //let total_claimed = TOTAL_CLAIMED.load(deps.storage)?;
            //let bank = deps
            //    .querier
            //    .query_balance(&env.contract.address, DENOM.load(deps.storage)?)?
            //    .amount
            //    .u128();
            //ensure!(
            //    total_registered - total_claimed <= bank,
            //    "Cannot register more than is banked."
            //);
            Ok(Response::new().add_event(
                Event::new("register")
                    .add_attribute("address", &tx_meta.address)
                    .add_attribute("claimable", Uint128::from(reward)),
            ))
        }
        ExecuteMsg::Claim {} => {
            let amount = REWARDS.load(deps.storage, info.sender.clone())?;
            REWARDS.remove(deps.storage, info.sender.clone());
            CLAIMED_REWARDS.update(deps.storage, info.sender.clone(), |total| {
                StdResult::Ok(total.unwrap_or(0) + amount)
            })?;
            TOTAL_CLAIMED.update(deps.storage, |claimed| StdResult::Ok(claimed + amount))?;
            Ok(Response::new()
                .add_message(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: coins(amount, DENOM.load(deps.storage)?),
                })
                .add_event(
                    Event::new("claim")
                        .add_attribute("address", info.sender)
                        .add_attribute("amount", Uint128::from(amount)),
                ))
        }
        ExecuteMsg::WithdrawAll {} => {
            ensure!(
                info.sender == ADMIN.load(deps.storage)?,
                "only admin can withdraw"
            );
            let bank = deps.querier.query_all_balances(env.contract.address)?;
            Ok(Response::new().add_message(BankMsg::Send {
                to_address: info.sender.into(),
                amount: bank,
            }))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&match msg {
        QueryMsg::Claim { address } => coin(
            REWARDS.may_load(deps.storage, address)?.unwrap_or(0),
            DENOM.load(deps.storage)?,
        ),
        QueryMsg::ClaimedRewards { address } => coin(
            CLAIMED_REWARDS.load(deps.storage, address)?,
            DENOM.load(deps.storage)?,
        ),
        QueryMsg::TotalClaimed {} => {
            coin(TOTAL_CLAIMED.load(deps.storage)?, DENOM.load(deps.storage)?)
        }
        QueryMsg::TotalRegistered {} => coin(
            TOTAL_REGISTERED.load(deps.storage)?,
            DENOM.load(deps.storage)?,
        ),
    })
}
