use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{Addr, Binary, Uint256};
use eyre::Result;
use secp256k1::hashes::sha256;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

use crate::contract::{execute, instantiate};
use crate::msg::{
    Consensus, ExecuteMsg, ExecutePayload, InstantiateMsg, LogicCallArgs, Valset, ValsetId,
};

fn keys(n: u64) -> (Vec<SecretKey>, Vec<Binary>, Vec<u32>) {
    let secp = Secp256k1::new();
    let (secret_keys, public_keys): (_, Vec<PublicKey>) =
        (0..n).map(|_| secp.generate_keypair(&mut OsRng)).unzip();
    (
        secret_keys,
        public_keys
            .iter()
            .map(|pk| Binary(pk.serialize().to_vec()))
            .collect(),
        (1..n).map(|_| ((1 << 32) / n) as u32).collect(),
    )
}

fn execute_msg(sks: &[SecretKey], msg: &ExecutePayload) -> Result<ExecuteMsg> {
    let secp = Secp256k1::new();
    let msg = serde_json::to_vec(&msg)?;
    let hash = Message::from_hashed_data::<sha256::Hash>(&msg);
    Ok(ExecuteMsg {
        consensus: Consensus {
            signatures: sks
                .iter()
                .map(|sk| {
                    Some(crate::msg::Signature(
                        secp.sign_ecdsa(&hash, sk).serialize_compact().to_vec(),
                    ))
                })
                .collect(),
        },
        payload: Binary(msg),
    })
}

#[test]
fn basic_workflow() -> Result<()> {
    // Validate total and maker fee bps
    let mut deps = mock_dependencies();
    let info = mock_info("admin0000", &[]);

    let (sks, validators, powers) = keys(8);
    let smart_contract_id = Addr::unchecked("contract0000");

    let _ = instantiate(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        InstantiateMsg {
            smart_contract_id: smart_contract_id.clone(),
            valset: Valset {
                valset_id: ValsetId(Uint256::zero()),
                validators,
                powers,
            },
        },
    )?;

    let (sks1, validators, powers) = keys(8);
    let _ = execute(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        execute_msg(
            &sks,
            &ExecutePayload::UpdateValset {
                valset: Valset {
                    valset_id: ValsetId(Uint256::from(1u8)),
                    validators,
                    powers,
                },
                smart_contract_id: smart_contract_id.clone(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    let sks = sks1;

    let payload = ExecutePayload::SubmitLogicCall {
        logic_call_args: LogicCallArgs {
            contract_address: Addr::unchecked("addr109"),
            payload: "".to_string(),
        },
        message_id: Uint256::from(42u8),
        smart_contract_id: smart_contract_id.clone(),
        deadline: mock_env().block.time.seconds() - 1,
    };
    let r = execute(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        execute_msg(&sks, &payload)?,
    );
    assert_eq!(&r.err().unwrap().to_string(), "Timeout");

    let payload = ExecutePayload::SubmitLogicCall {
        logic_call_args: LogicCallArgs {
            contract_address: Addr::unchecked("addr109"),
            payload: "".to_string(),
        },
        message_id: Uint256::from(42u8),
        smart_contract_id,
        deadline: mock_env().block.time.seconds() + 1,
    };
    let r = execute(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        execute_msg(&sks[..2], &payload)?,
    );
    assert_eq!(&r.err().unwrap().to_string(), "Insufficient Power");

    let r = execute(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        execute_msg(&sks, &payload)?,
    )?;
    assert_eq!(r.messages.len(), 1);

    let r = execute(
        deps.as_mut(),
        mock_env(),
        info,
        execute_msg(&sks, &payload)?,
    );
    assert_eq!(r.err().unwrap().to_string(), "Used Message_ID");

    Ok(())
}
