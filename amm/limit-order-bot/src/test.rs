use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, CosmosMsg, DepsMut};
use eyre::Result;

use crate::contract::{execute, instantiate};
use crate::msg::{CustomResponseMsg, ExecuteMsg, InstantiateMsg, TargetContractInfo};

#[test]
fn simple_contest() -> Result<()> {
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
            lower_tick: 1000,
            deadline: 1000000000000,
        },
    )?;

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &coins(1000000, "ugrain")),
        ExecuteMsg::GetDeposit {
            token_id: 1,
            lower_tick: 1000,
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
    assert_ne!(t.payload, "".to_string());

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
    assert_ne!(t.payload, "".to_string());

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin0000", &[]),
        ExecuteMsg::GetWithdraw { token_ids: vec![1] },
    )?;
    Ok(())
}
