use crate::contract::{execute, instantiate};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, Addr, Attribute, BankMsg, CosmosMsg, Uint128};
use eyre::Result;

use crate::msg::{ExecuteMsg, InstantiateMsg};

#[test]
fn full_flow() -> Result<()> {
    // Validate total and maker fee bps
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        claims: vec![
            (Addr::unchecked("p1"), Uint128::from(4u8)),
            (Addr::unchecked("p2"), Uint128::from(5u8)),
            (Addr::unchecked("p3"), Uint128::from(6u8)),
        ],
    };
    let info = mock_info("admin0000", &coins(15, "ucarrot"));
    let _ = instantiate(deps.as_mut(), mock_env(), info, msg)?;

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("p1", &[]),
        ExecuteMsg::Claim {},
    )?;
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("p2", &[]),
        ExecuteMsg::Claim {},
    )?;
    let r = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("admin0000", &[]),
        ExecuteMsg::Clear {},
    )?;
    assert_eq!(
        r.attributes[0],
        Attribute {
            key: "p3".to_string(),
            value: "6".to_string(),
        }
    );
    match &r.messages[0].msg.clone() {
        CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, "admin0000");
            assert_eq!(amount, &coins(6, "ucarrot"));
        }
        _ => panic!("Weird messages."),
    }

    Ok(())
}
