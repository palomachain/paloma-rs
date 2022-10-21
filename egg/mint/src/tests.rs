use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, Binary, DepsMut};
use eyre::Result;
use std::collections::HashMap;

use crate::contract::{execute, instantiate, ENTRANCE_FEE};
use crate::msg::{ExecuteMsg, InstantiateMsg};

fn add_entrant(deps: DepsMut, n: u16, funds: u128) -> Result<()> {
    execute(
        deps,
        mock_env(),
        mock_info(&format!("addr{:04}", n), &coins(funds, "ugrain")),
        ExecuteMsg::LayEgg {
            eth_address: format!("0x000000000000000000000000000000000000{:04}", n),
        },
    )?;
    Ok(())
}

#[test]
fn simple_contest() -> Result<()> {
    // Validate total and maker fee bps
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        job_id: "job".to_string(),
    };
    let info = mock_info("admin0000", &[]);
    let _ = instantiate(deps.as_mut(), mock_env(), info, msg)?;

    // If we add someone they'll win.
    add_entrant(deps.as_mut(), 0, ENTRANCE_FEE)?;
    let r = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin0000", &[]),
        ExecuteMsg::PickWinner {
            payload: Binary(vec![]),
        },
    )?;
    let attributes: HashMap<_, _> = r
        .attributes
        .iter()
        .map(|att| (att.key.clone(), att.value.clone()))
        .collect();
    assert_eq!(attributes["winning_paloma_address"], "addr0000");
    assert_eq!(
        attributes["winning_eth_address"],
        "0x0000000000000000000000000000000000000000"
    );

    // Entrant 0 can never win again
    add_entrant(deps.as_mut(), 1, ENTRANCE_FEE)?;
    let r = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin0000", &[]),
        ExecuteMsg::PickWinner {
            payload: Binary(vec![]),
        },
    )?;
    let attributes: HashMap<_, _> = r
        .attributes
        .iter()
        .map(|att| (att.key.clone(), att.value.clone()))
        .collect();
    assert_eq!(attributes["winning_paloma_address"], "addr0001");
    assert_eq!(
        attributes["winning_eth_address"],
        "0x0000000000000000000000000000000000000001"
    );

    Ok(())
}

#[test]
fn simple_errors() -> Result<()> {
    // Validate total and maker fee bps
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        job_id: "job".to_string(),
    };
    let info = mock_info("admin0000", &[]);
    let _ = instantiate(deps.as_mut(), mock_env(), info, msg)?;

    // Not enough funds.
    assert!(add_entrant(deps.as_mut(), 0, 5).is_err());

    // Not the admin.
    assert!(execute(
        deps.as_mut(),
        mock_env(),
        mock_info("addr0000", &[]),
        ExecuteMsg::PickWinner {
            payload: Binary(vec![])
        },
    )
    .is_err());

    // If we add someone...
    add_entrant(deps.as_mut(), 0, ENTRANCE_FEE)?;
    // and they win...
    let _ = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin0000", &[]),
        ExecuteMsg::PickWinner {
            payload: Binary(vec![]),
        },
    )?;
    // We won't let them re-enter with those addresses.
    assert!(add_entrant(deps.as_mut(), 0, ENTRANCE_FEE).is_err());

    Ok(())
}
