use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{ADMIN, DELEGATE_ADDRESS, DENOM, REWARDS, SUBMITTED, TOTAL_CLAIMED};
use cosmwasm_std::{
    coin, coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
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
            let amount = REWARDS
                .may_load(deps.storage, tx_meta.address.clone())?
                .unwrap_or(0);
            let amount = amount + tx_meta.amount.u128();
            REWARDS.save(deps.storage, tx_meta.address, &amount)?;
            Ok(Response::new().add_attribute("claimable", Uint128::from(amount)))
        }
        ExecuteMsg::Claim {} => {
            let amount = REWARDS.load(deps.storage, info.sender.clone())?;
            REWARDS.remove(deps.storage, info.sender.clone());
            let total = TOTAL_CLAIMED
                .may_load(deps.storage, info.sender.clone())?
                .unwrap_or(0);
            let total = total + amount;
            TOTAL_CLAIMED.save(deps.storage, info.sender.clone(), &total)?;

            Ok(Response::new().add_message(BankMsg::Send {
                to_address: info.sender.into(),
                amount: coins(amount, DENOM.load(deps.storage)?),
            }))
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
    match msg {
        QueryMsg::Claim { address } => to_binary(&coin(
            REWARDS.may_load(deps.storage, address)?.unwrap_or(0),
            DENOM.load(deps.storage)?,
        )),
        QueryMsg::TotalClaimed { address } => to_binary(&coin(
            TOTAL_CLAIMED.load(deps.storage, address)?,
            DENOM.load(deps.storage)?,
        )),
    }
}
