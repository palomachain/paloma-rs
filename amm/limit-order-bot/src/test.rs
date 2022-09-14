// use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{to_binary, Addr, Binary, Empty, Response, StdResult};
use cw_multi_test::{custom_app, BasicApp, Contract, ContractWrapper, Executor};
use eyre::Result;
use schemars::JsonSchema;

use crate::contract::{execute, instantiate, query};
use crate::msg::PythBridgeQueryMsg::PriceFeed;
use crate::msg::{
    CustomResponseMsg, ExecuteMsg, InstantiateMsg, PriceFeedResponse, PythBridgeQueryMsg, QueryMsg,
    TargetContractInfo, TokenIdList,
};
use crate::ContractError;

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
            smart_contract_abi: "".to_string(),
        },
        price_contract: "".to_string(),
    };
    let price_contract_address = router
        .instantiate_contract(
            price_contract_code_id,
            owner.clone(),
            &init_msg,
            &[],
            "price_contract",
            None,
        )
        .unwrap();
    let init_msg = InstantiateMsg {
        target_contract_info: TargetContractInfo {
            method: "".to_string(),
            chain_id: "".to_string(),
            compass_id: "".to_string(),
            contract_address: "".to_string(),
            smart_contract_abi: "".to_string(),
        },
        price_contract: price_contract_address.to_string(),
    };

    let main_contract_id = router.store_code(contract_main());
    let mocked_main_contract_addr = router
        .instantiate_contract(
            main_contract_id,
            owner.clone(),
            &init_msg,
            &[],
            "main contract",
            None,
        )
        .unwrap();

    let msg = ExecuteMsg::GetDeposit {
        token_id: 0,
        sqrt_price_x96: Default::default(),
        deadline: 0,
    };
    let _ = router
        .execute_contract(owner, mocked_main_contract_addr.clone(), &msg, &[])
        .unwrap();
    let msg = QueryMsg::DepositList {};
    let result: TokenIdList = router
        .wrap()
        .query_wasm_smart(mocked_main_contract_addr, &msg)
        .unwrap();
    assert_eq!(result.list.len(), 1);
    Ok(())
}

pub fn contract_price_mock<T>() -> Box<dyn Contract<T>>
where
    ContractWrapper<
        ExecuteMsg,
        InstantiateMsg,
        PythBridgeQueryMsg,
        cosmwasm_std::StdError,
        cosmwasm_std::StdError,
        cosmwasm_std::StdError,
        CustomResponseMsg,
    >: Contract<T>,
    T: Clone + std::fmt::Debug + PartialEq + JsonSchema,
{
    let contract: ContractWrapper<
        ExecuteMsg,
        InstantiateMsg,
        PythBridgeQueryMsg,
        cosmwasm_std::StdError,
        cosmwasm_std::StdError,
        cosmwasm_std::StdError,
        CustomResponseMsg,
    > = ContractWrapper::new(
        |_, _, _, _| -> StdResult<Response<CustomResponseMsg>> { Ok(Response::new()) },
        |_, _, _, _| -> StdResult<Response<CustomResponseMsg>> { Ok(Response::new()) },
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

pub fn contract_main<T>() -> Box<dyn Contract<T>>
where
    ContractWrapper<
        ExecuteMsg,
        InstantiateMsg,
        QueryMsg,
        ContractError,
        ContractError,
        cosmwasm_std::StdError,
        CustomResponseMsg,
    >: Contract<T>,
    T: Clone + std::fmt::Debug + PartialEq + JsonSchema,
{
    let contract: ContractWrapper<_, _, _, _, _, _, CustomResponseMsg> =
        ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

fn mock_app() -> BasicApp<CustomResponseMsg> {
    custom_app::<CustomResponseMsg, Empty, _>(|_, _, _| {})
}
