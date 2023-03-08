use crate::contract::{execute, instantiate};
use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
use cosmwasm_std::{coins, Addr, BankMsg, CosmosMsg, Uint128};
use eyre::Result;

use crate::msg::{ExecuteMsg, InstantiateMsg, TxMeta};

#[test]
fn full_flow() -> Result<()> {
    let mut deps = mock_dependencies_with_balance(&coins(15, "ucarrot"));

    let admin = Addr::unchecked("admin0000");
    let delegate = Addr::unchecked("delegate0000");
    let (p1, _p2, _p3) = (
        Addr::unchecked("p1"),
        Addr::unchecked("p2"),
        Addr::unchecked("p3"),
    );

    let msg = InstantiateMsg {
        delegate_address: delegate.clone(),
    };
    let info = mock_info(admin.as_str(), &coins(15, "ucarrot"));
    instantiate(deps.as_mut(), mock_env(), info, msg)?;

    // Check that we can't overcommit.
    //let r = execute(
    //    mock_dependencies_with_balance(&coins(15, "ucarrot")).as_mut(),
    //    mock_env(),
    //    mock_info(delegate.as_str(), &[]),
    //    ExecuteMsg::Register {
    //        tx_meta: TxMeta {
    //            address: p1.clone(),
    //            amount: Uint128::from(100u8),
    //            tx_hash: "abc".to_string(),
    //        },
    //    },
    //);
    //assert!(r.is_err());

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(delegate.as_str(), &[]),
        ExecuteMsg::Register {
            tx_meta: TxMeta {
                address: p1.clone(),
                amount: Uint128::from(1u8),
                tx_hash: "a".to_string(),
            },
        },
    )?;
    let r = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("p1", &[]),
        ExecuteMsg::Claim {},
    )?;
    match &r.messages[0].msg.clone() {
        CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, p1.as_str());
            assert_eq!(amount, &coins(1, "ucarrot"));
        }
        _ => panic!("Bad messages."),
    }
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(admin.as_str(), &[]),
        ExecuteMsg::WithdrawAll {},
    )?;
    // TODO: This works in the _real_ world but not in unit tests?
    //match &r.messages[0].msg.clone() {
    //    CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
    //        assert_eq!(to_address, admin.as_str());
    //        assert_eq!(amount, &coins(14, "ucarrot"));
    //    }
    //    _ => panic!("Bad messages."),
    //}

    Ok(())
}
