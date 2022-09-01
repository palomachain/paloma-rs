#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CustomResponseMsg, ExecuteMsg, InstantiateMsg, MultipleIdMsg, QueryMsg, SingleIdMsg, TargetContractInfo, TokenIdList};
use crate::state::{Deposit, DEPOSIT, PRICE, TARGET_CONTRACT_INFO};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:limit-order-bot";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    TARGET_CONTRACT_INFO.save(deps.storage, &msg.target_contract_info)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::GetDeposit {
            token_id,
            lower_tick,
            depositor,
            deadline,
        } => get_deposit(deps, token_id, lower_tick, depositor, deadline),
        ExecuteMsg::PutWithdraw {} => put_withdraw(deps),
        ExecuteMsg::GetWithdraw { token_ids } => get_withdraw(deps, token_ids),
        ExecuteMsg::PutCancel {} => put_cancel(deps, env),
        ExecuteMsg::GetCancel { token_ids } => get_cancel(deps, token_ids),
    }
}

fn get_deposit(
    deps: DepsMut,
    token_id: u128,
    lower_tick: i32,
    depositor: String,
    deadline: u64,
) -> Result<Response, ContractError> {
    DEPOSIT.save(
        deps.storage,
        token_id,
        &Deposit {
            lower_tick,
            deadline,
        },
    )?;
    Ok(Response::new())
}

fn put_withdraw(deps: DepsMut) -> Result<Response, ContractError> {
    let range = DEPOSIT.range(deps.storage, None, None, Order::Ascending);
    let price = PRICE.load(deps.storage).unwrap_or_default(); // TODO: need to set a price or get a price from the other sc
    if price == 0 {
        return Err(ContractError::CustomError {
            val: "Price is not set.".to_string(),
        });
    }
    let tick = price2tick(price) + 50; // ERR 0.5%
    let mut token_ids: Vec<u128> = Vec::new();
    for x in range {
        let x = x?;
        if tick < x.1.lower_tick {
            token_ids.push(x.0);
        }
    }
    let target_contract_info = TARGET_CONTRACT_INFO.load(deps.storage)?;
    if token_ids.len() > 1 {
        Ok(Response::new().add_message(CosmosMsg::Custom(MultipleIdMsg {
            target_contract_info,
            method: "multiple_withdraw(uint256[])".to_string(),
            token_ids,
        })))
    } else if token_ids.len() == 1 {
        Ok(Response::new().add_message(CosmosMsg::Custom(SingleIdMsg {
            target_contract_info,
            method: "withdraw(uint256)".to_string(),
            token_id:token_ids[0],
        })))
    } else {
        Err(ContractError::CustomError {
            val: "Nothing to withdraw".to_string(),
        })
    }
}

fn get_withdraw(deps: DepsMut, token_ids: Vec<u128>) -> Result<Response, ContractError> {
    for token_id in token_ids {
        DEPOSIT.remove(deps.storage, token_id);
    }
    Ok(Response::new())
}

fn put_cancel(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let range = DEPOSIT.range(deps.storage, None, None, Order::Ascending);
    let mut token_ids: Vec<u128> = Vec::new();
    for x in range {
        let x = x?;
        if env.block.time.seconds() < x.1.deadline {
            token_ids.push(x.0);
        }
    }
    if token_ids.len() > 1 {
        Ok(Response::new().add_message(CosmosMsg::Custom(MultipleIdMsg {
            target_contract_info,
            method: "multiple_cancel(uint256[])".to_string(),
            token_ids,
        })))
    } else if token_ids.len() == 1 {
        Ok(Response::new().add_message(CosmosMsg::Custom(SingleIdMsg {
            target_contract_info,
            method: "cancel(uint256)".to_string(),
            token_id:token_ids[0],
        })))
    } else {
        Err(ContractError::CustomError {
            val: "Nothing to withdraw".to_string(),
        })
    }
}

fn get_cancel(deps: DepsMut, token_ids: Vec<u128>) -> Result<Response, ContractError> {
    for token_id in token_ids {
        DEPOSIT.remove(deps.storage, token_id);
    }
    Ok(Response::new())
}

fn price2tick(price: f32) -> i32 {
    let ratio = 1_000_000_000_000.0 / price;
    ratio.log(1.0001).floor() as i32
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::DepositList {} => to_binary(&deposit_list(deps)?),
        QueryMsg::WithdrawableList {} => to_binary(&withdrawable_list(deps)?),
    }
}

fn deposit_list(deps: Deps) -> StdResult<TokenIdList> {
    let keys = DEPOSIT.keys(deps.storage, None, None, Order::Ascending);
    let mut token_ids: Vec<u128> = Vec::new();
    for key in keys {
        token_ids.push(key?);
    }
    Ok(TokenIdList { list: token_ids })
}

fn withdrawable_list(deps: Deps) -> StdResult<TokenIdList> {
    let range = DEPOSIT.range(deps.storage, None, None, Order::Ascending);
    let price = PRICE.load(deps.storage).unwrap_or_default(); // or get price
    assert_eq!(price, 0);
    let tick = price2tick(price) + 50; // ERR 0.5%
    let mut token_ids: Vec<u128> = Vec::new();
    for x in range {
        let x = x?;
        if tick < x.1.lower_tick {
            token_ids.push(x.0);
        }
    }
    Ok(TokenIdList { list: token_ids })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
