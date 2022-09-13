use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, to_binary, Binary, CosmosMsg, Empty, Response, StdResult, Addr};
use cw_multi_test::{App, BasicApp, Contract, ContractWrapper, custom_app, Executor};
use eyre::Result;

use crate::contract::{execute, instantiate, query};
use crate::msg::PythBridgeQueryMsg::PriceFeed;
use crate::msg::{ExecuteMsg, InstantiateMsg, PriceFeedResponse, PythBridgeQueryMsg, QueryMsg, TargetContractInfo};

#[test]
fn simple_contest() -> Result<()> {
    let owner = Addr::unchecked("owner");
    let mut router = mock_app();
    let price_contract_code_id = router.store_code(contract_price_mock());
    let init_msg = InstantiateMsg {
        target_contract_info: TargetContractInfo {
            method: "".to_string(),
            chain_id: "".to_string(),
            compass_id: "".to_string(),
            contract_address: "".to_string(),
            smart_contract_abi: "".to_string()
        },
        price_contract: "".to_string()
    };
    let price_contract_address = router.instantiate_contract(price_contract_code_id, owner.clone(), &init_msg, &[], "price_contract", None).unwrap();


    // Validate total and maker fee bps
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        target_contract_info: TargetContractInfo {
            method: "".to_string(),
            chain_id: "".to_string(),
            compass_id: "".to_string(),
            contract_address: "".to_string(),
            smart_contract_abi: "".to_string(),
        },
        price_contract: "".to_string(),
    };
    let info = mock_info("admin0000", &[]);
    let _ = instantiate(deps.as_mut(), mock_env(), info, msg)?;

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &coins(1_000_000, "ugrain")),
        ExecuteMsg::GetDeposit {
            token_id: 0,
            sqrt_price_x96: Default::default(),
            deadline: 1000000000000,
        },
    )?;

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &coins(1000000, "ugrain")),
        ExecuteMsg::GetDeposit {
            token_id: 1,
            sqrt_price_x96: Default::default(),
            deadline: 1000,
        },
    )?;

    let r = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin0000", &[]),
        ExecuteMsg::PutWithdraw {},
    )?;
    let msg = r.messages.first().unwrap().msg.clone();
    let t = if let CosmosMsg::Custom(t) = msg {
        t
    } else {
        todo!()
    };
    assert_ne!(t.payload, Binary::default());

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin0000", &[]),
        ExecuteMsg::GetWithdraw { token_ids: vec![0] },
    )?;

    let r = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin0000", &[]),
        ExecuteMsg::PutWithdraw {},
    )?;
    let msg = r.messages.first().unwrap().msg.clone();
    let t = if let CosmosMsg::Custom(t) = msg {
        t
    } else {
        todo!()
    };
    assert_ne!(t.payload, Binary::default());

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin0000", &[]),
        ExecuteMsg::GetWithdraw { token_ids: vec![1] },
    )?;
    Ok(())
}

pub fn contract_price_mock() -> Box<dyn Contract<Empty>> {
    let contract :ContractWrapper<ExecuteMsg, InstantiateMsg, PythBridgeQueryMsg, cosmwasm_std::StdError, cosmwasm_std::StdError, cosmwasm_std::StdError> = ContractWrapper::new(
        |_, _, _, _| -> StdResult<Response> { Ok(Response::new()) },
        |_, _, _, _| -> StdResult<Response> { Ok(Response::new()) },
        |_, _, msg| -> StdResult<Binary> {
            match msg {
                PriceFeed { .. } => to_binary(&PriceFeedResponse {
                    price_feed: Default::default(),
                }),
            }
        },
    );
    Box::new(contract)
}

pub fn contract_main() -> Box<dyn Contract<Empty>> {
    let contract :ContractWrapper<ExecuteMsg, InstantiateMsg, QueryMsg, cosmwasm_std::StdError, cosmwasm_std::StdError, cosmwasm_std::StdError> = ContractWrapper::new(
        execute,
        instantiate,
        query,
    );
    Box::new(contract)
}

fn mock_app() -> App {
    App::default()
}
