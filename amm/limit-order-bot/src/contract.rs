#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order, QueryRequest, Response,
    StdResult, WasmQuery,
};
use cw2::set_contract_version;
use ethabi::{Contract, Function, Param, ParamType, StateMutability, Token, Uint};
use hex::encode;
use pyth_bridge::msg::PriceFeedResponse;
use pyth_bridge::msg::QueryMsg::PriceFeed;
use pyth_sdk::PriceIdentifier;
use std::collections::BTreeMap;

use crate::error::ContractError;
use crate::msg::{CustomResponseMsg, ExecuteMsg, InstantiateMsg, QueryMsg, TokenIdList};
use crate::state::{Deposit, DEPOSIT, ETH_USD, PRICE_CONTRACT, TARGET_CONTRACT_INFO};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:limit-order-bot";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<CustomResponseMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    TARGET_CONTRACT_INFO.save(deps.storage, &msg.target_contract_info)?;
    PRICE_CONTRACT.save(deps.storage, &msg.price_contract)?; // paloma1xr3rq8yvd7qplsw5yx90ftsr2zdhg4e9z60h5duusgxpv72hud3sac3fdu
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<CustomResponseMsg>, ContractError> {
    match msg {
        ExecuteMsg::GetDeposit {
            token_id,
            lower_tick,
            deadline,
        } => get_deposit(deps, token_id, lower_tick, deadline),
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
    deadline: u64,
) -> Result<Response<CustomResponseMsg>, ContractError> {
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

fn put_withdraw(deps: DepsMut) -> Result<Response<CustomResponseMsg>, ContractError> {
    let tick = get_tick(deps.as_ref());
    let range = DEPOSIT.range(deps.storage, None, None, Order::Ascending);
    let mut token_ids: Vec<u128> = Vec::new();
    for item in range {
        let (token_id, deposit) = item.unwrap();
        if tick < deposit.lower_tick {
            token_ids.push(token_id);
        }
    }
    let target_contract_info = TARGET_CONTRACT_INFO.load(deps.storage)?;
    #[allow(deprecated)]
    let contract: Contract = Contract {
        constructor: None,
        functions: BTreeMap::from_iter(vec![
            (
                "withdraw".to_string(),
                vec![Function {
                    name: "withdraw".to_string(),
                    inputs: vec![Param {
                        name: "tokenId".to_string(),
                        kind: ParamType::Uint(256),
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            ),
            (
                "multiple_withdraw".to_string(),
                vec![Function {
                    name: "multiple_withdraw".to_string(),
                    inputs: vec![Param {
                        name: "tokenIds".to_string(),
                        kind: ParamType::Array(Box::new(ParamType::Uint(256))),
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            ),
        ]),
        events: BTreeMap::new(),
        errors: BTreeMap::new(),
        receive: false,
        fallback: false,
    };
    match token_ids.len() {
        0 => Err(ContractError::CustomError {
            val: "Nothing to withdraw".to_string(),
        }),
        1 => Ok(
            Response::new().add_message(CosmosMsg::Custom(CustomResponseMsg {
                target_contract_info,
                payload: encode(
                    contract
                        .function("withdraw")
                        .unwrap()
                        .encode_input(&[Token::Uint(Uint::from(token_ids[0]))])
                        .unwrap(),
                ),
            })),
        ),
        _ => {
            let mut tokens = Vec::new();
            for token_id in token_ids {
                tokens.push(Token::Uint(Uint::from(token_id)));
            }
            Ok(
                Response::new().add_message(CosmosMsg::Custom(CustomResponseMsg {
                    target_contract_info,
                    payload: encode(
                        contract
                            .function("multiple_withdraw")
                            .unwrap()
                            .encode_input(tokens.as_slice())
                            .unwrap(),
                    ),
                })),
            )
        }
    }
}

fn get_withdraw(
    deps: DepsMut,
    token_ids: Vec<u128>,
) -> Result<Response<CustomResponseMsg>, ContractError> {
    for token_id in token_ids {
        DEPOSIT.remove(deps.storage, token_id);
    }
    Ok(Response::new())
}

fn put_cancel(deps: DepsMut, env: Env) -> Result<Response<CustomResponseMsg>, ContractError> {
    let range = DEPOSIT.range(deps.storage, None, None, Order::Ascending);
    let mut token_ids: Vec<u128> = Vec::new();
    for x in range {
        let x = x?;
        if env.block.time.seconds() < x.1.deadline {
            token_ids.push(x.0);
        }
    }
    let target_contract_info = TARGET_CONTRACT_INFO.load(deps.storage)?;
    #[allow(deprecated)]
    let contract: Contract = Contract {
        constructor: None,
        functions: BTreeMap::from_iter(vec![
            (
                "cancel".to_string(),
                vec![Function {
                    name: "cancel".to_string(),
                    inputs: vec![Param {
                        name: "tokenId".to_string(),
                        kind: ParamType::Uint(256),
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            ),
            (
                "multiple_cancel".to_string(),
                vec![Function {
                    name: "multiple_cancel".to_string(),
                    inputs: vec![Param {
                        name: "tokenIds".to_string(),
                        kind: ParamType::Array(Box::new(ParamType::Uint(256))),
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            ),
        ]),
        events: BTreeMap::new(),
        errors: BTreeMap::new(),
        receive: false,
        fallback: false,
    };

    match token_ids.len() {
        0 => Err(ContractError::CustomError {
            val: "Nothing to withdraw".to_string(),
        }),
        1 => Ok(
            Response::new().add_message(CosmosMsg::Custom(CustomResponseMsg {
                target_contract_info,
                payload: encode(
                    contract
                        .function("withdraw")
                        .unwrap()
                        .encode_input(&[Token::Uint(Uint::from(token_ids[0]))])
                        .unwrap(),
                ),
            })),
        ),
        _ => {
            let mut tokens = Vec::new();
            for token_id in token_ids {
                tokens.push(Token::Uint(Uint::from(token_id)));
            }
            Ok(
                Response::new().add_message(CosmosMsg::Custom(CustomResponseMsg {
                    target_contract_info,
                    payload: encode(
                        contract
                            .function("multiple_cancel")
                            .unwrap()
                            .encode_input(tokens.as_slice())
                            .unwrap(),
                    ),
                })),
            )
        }
    }
}

fn get_cancel(
    deps: DepsMut,
    token_ids: Vec<u128>,
) -> Result<Response<CustomResponseMsg>, ContractError> {
    for token_id in token_ids {
        DEPOSIT.remove(deps.storage, token_id);
    }
    Ok(Response::new())
}

fn get_tick(deps: Deps) -> i32 {
    let pyth_bridge_contract = PRICE_CONTRACT.load(deps.storage).unwrap();
    let vaa: PriceFeedResponse = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: pyth_bridge_contract,
            msg: to_binary(&PriceFeed {
                id: PriceIdentifier::from_hex(ETH_USD).unwrap(),
            })
            .unwrap(),
        }))
        .unwrap();
    let price = vaa.price_feed.get_current_price().unwrap_or_default();
    assert_ne!(price.price, 0);
    let price = (price.price as f64) * 10_f64.powi(price.expo);
    price2tick(price) + 50 // ERR 0.5%
}

fn price2tick(price: f64) -> i32 {
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
    Ok(TokenIdList {
        list: keys.into_iter().collect::<StdResult<_>>()?,
    })
}

fn withdrawable_list(deps: Deps) -> StdResult<TokenIdList> {
    let tick = get_tick(deps);
    let mut token_ids: Vec<u128> = Vec::new();
    let range = DEPOSIT.range(deps.storage, None, None, Order::Ascending);
    for x in range {
        let x = x?;
        if tick < x.1.lower_tick {
            token_ids.push(x.0);
        }
    }
    Ok(TokenIdList { list: token_ids })
}
