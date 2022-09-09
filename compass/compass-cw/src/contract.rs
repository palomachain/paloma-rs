use crate::msg::{Consensus, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, Valset};
use crate::msg::{LogicCallArgs, Signature};
use crate::state::{MESSAGE_ID_USED, SMART_CONTRACT_ID, VALSET, VALSET_ID};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256, WasmMsg,
};
use eyre::{bail, ensure, Result};
use itertools::izip;
use ring::digest::{digest, SHA256};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

/// 2/3 * 2**32.
/// Validator powers will be normalized to sum to 2**32 in every valset update.
const POWER_THRESHOLD: u64 = 2_863_311_530;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response> {
    check_validator_power(&msg.valset.powers)?;
    SMART_CONTRACT_ID.save(deps.storage, &msg.smart_contract_id)?;
    VALSET.save(deps.storage, &msg.valset)?;
    VALSET_ID.save(deps.storage, &msg.valset.valset_id)?;
    Ok(Response::new())
}

fn check_signature(
    deps: Deps,
    public_key: &[u8],
    message_hash: &[u8],
    sig: &Signature,
) -> Result<()> {
    ensure!(
        deps.api.secp256k1_verify(message_hash, &sig.0, public_key) == Ok(true),
        "Invalid Signature"
    );
    Ok(())
}

fn check_validator_power(powers: &[u32]) -> Result<()> {
    let mut cumulative_power: u64 = 0;
    for &power in powers {
        cumulative_power += power as u64;
        if cumulative_power >= POWER_THRESHOLD {
            return Ok(());
        }
    }
    bail!("Insufficient Power");
}

fn check_validator_signatures(deps: Deps, consensus: &Consensus, msg: &[u8]) -> Result<()> {
    let valset = VALSET.load(deps.storage)?;
    let hash = digest(&SHA256, msg);
    let mut cumulative_power: u64 = 0;
    for (validator, &power, sig) in izip!(&valset.validators, &valset.powers, &consensus.signatures)
    {
        if let Some(sig) = sig {
            check_signature(deps, validator, hash.as_ref(), sig)?;
            cumulative_power += power as u64;
            if cumulative_power >= POWER_THRESHOLD {
                return Ok(());
            }
        }
    }
    bail!("Insufficient Power");
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response> {
    check_validator_signatures(deps.as_ref(), &msg.consensus, &msg.payload)?;
    let payload = serde_json::from_slice(&msg.payload)?;
    let id = SMART_CONTRACT_ID.load(deps.storage)?;

    use crate::msg::ExecutePayload::*;
    match payload {
        UpdateValset {
            valset: new_valset,
            smart_contract_id,
        } => {
            ensure!(smart_contract_id == id, "Wrong smart contract instance");
            update_valset(deps, env, info, &new_valset)
        }
        SubmitLogicCall {
            logic_call_args,
            message_id,
            smart_contract_id,
            deadline,
        } => {
            ensure!(smart_contract_id == id, "Wrong smart contract instance");
            submit_logic_call(deps, env, info, logic_call_args, message_id, deadline)
        }
    }
}

/// This updates the valset by checking that the validators in the current valset have signed off on the
/// new valset.
/// Anyone can call this function, but they must supply valid signatures of constant_powerThreshold of the current valset over
/// the new valset.
/// valset: new validator set to update with
/// consensus: current validator set and signatures
fn update_valset(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    new_valset: &Valset,
) -> Result<Response> {
    let valset_id = VALSET_ID.load(deps.storage)?;
    ensure!(
        new_valset.valset_id > valset_id,
        "Valset ID must be greater than the current valset ID"
    );
    check_validator_power(&new_valset.powers)?;
    VALSET.save(deps.storage, new_valset)?;
    VALSET_ID.save(deps.storage, &new_valset.valset_id)?;
    Ok(Response::new())
}

/// This makes calls to contracts that execute arbitrary logic
/// message_id is to prevent replay attack and every message_id can be used only once
fn submit_logic_call(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    args: LogicCallArgs,
    message_id: Uint256,
    deadline: u64,
) -> Result<Response> {
    ensure!(
        args.contract_address != env.contract.address,
        "Probable error, recursive compass invocation"
    );
    ensure!(env.block.time.seconds() < deadline, "Timeout");
    ensure!(
        info.funds.iter().all(|coin| coin.amount.is_zero()),
        "No funds should be sent to compass"
    );
    let message_id_bytes = message_id.to_be_bytes().to_vec();
    ensure!(
        !MESSAGE_ID_USED.has(deps.storage, message_id_bytes.clone()),
        "Used Message_ID"
    );
    MESSAGE_ID_USED.save(deps.storage, message_id_bytes, &())?;
    let LogicCallArgs {
        contract_address: logic_contract_address,
        payload,
    } = args;
    Ok(Response::new().add_message(WasmMsg::Execute {
        contract_addr: logic_contract_address.into_string(),
        msg: Binary(payload.into_bytes()),
        funds: vec![],
    }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SmartContractId => to_binary(&SMART_CONTRACT_ID.load(deps.storage)?),
        QueryMsg::ValsetId => to_binary(&VALSET_ID.load(deps.storage)?),
    }
}
