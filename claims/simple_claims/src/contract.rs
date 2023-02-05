use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{ADMIN, BANK, DENOM, REGISTER};
use cosmwasm_std::{
    coin, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};
use eyre::{ensure, Result};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Order::Ascending;

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
) -> Result<Response> {
    ADMIN.save(deps.storage, &info.sender)?;
    ensure!(info.funds.len() == 1, "only one funds slot supported");
    let Coin { denom, amount } = &info.funds[0];
    DENOM.save(deps.storage, denom)?;

    let mut total = Uint128::zero();
    for (addr, amt) in msg.claims {
        REGISTER.save(deps.storage, addr, &amt)?;
        total += amt;
    }
    ensure!(
        total == amount,
        "Provided funds must be exactly what is distributed."
    );
    BANK.save(deps.storage, &total)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response> {
    match msg {
        ExecuteMsg::Claim {} => {
            let amount = REGISTER.load(deps.storage, info.sender.clone())?;
            REGISTER.remove(deps.storage, info.sender.clone());
            let bank = BANK.load(deps.storage)?;
            BANK.save(deps.storage, &(bank - amount))?;
            Ok(Response::new().add_message(BankMsg::Send {
                to_address: info.sender.into(),
                amount: vec![coin(amount.u128(), DENOM.load(deps.storage)?)],
            }))
        }
        ExecuteMsg::Clear {} => {
            let admin = ADMIN.load(deps.storage)?;
            ensure!(info.sender == admin, "only admin can add claims");
            let mut res = Response::new();
            for entry in REGISTER.range(deps.storage, None, None, Ascending) {
                let (addr, amt) = entry?;
                res = res.add_attribute(&addr, amt);
            }
            Ok(res.add_message(BankMsg::Send {
                to_address: admin.into(),
                amount: vec![coin(
                    BANK.load(deps.storage)?.u128(),
                    DENOM.load(deps.storage)?,
                )],
            }))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&())
}
