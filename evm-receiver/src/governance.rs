use crate::messages::{parse_vm, verify_vm};
use crate::state::{CONSUMED_GOVERNANCE_ACTIONS, GUARDIAN_SETS, GUARDIAN_SET_INDEX, OWNER};
use crate::structs::VM;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo};
use ethabi::Address;
use eyre::{ensure, Result};

// XXX: import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Upgrade.sol";

// XXX: abstract contract Governance is GovernanceStructs, Messages, Setters, ERC1967Upgrade {
// XXX: event ContractUpgraded(address indexed oldContract, address indexed newContract);
// XXX: event OwnershipTransfered(address indexed oldOwner, address indexed newOwner);

// "Core" (left padded)
const module: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x43, 0x6f,
    0x72, 0x65,
];

fn submit_new_guardian_set(deps: DepsMut, env: Env, encoded_vm: &[u8]) -> Result<()> {
    let vm = parse_vm(encoded_vm)?;
    verify_governance_vm(&vm)?;

    let upgrade = parse_guardian_set_upgrade(&vm.payload)?;
    ensure!(upgrade.module == module, "invalid Module");
    ensure!(
        upgrade.new_guardian_set.keys.length > 0,
        "new guardian set is empty",
    );

    let guardian_set_index = GUARDIAN_SET_INDEX.load(deps.storage)?;
    ensure!(
        upgrade.new_guardian_set_index == guardian_set_index + 1,
        "index must increase in steps of 1",
    );

    CONSUMED_GOVERNANCE_ACTIONS.store(vm.hash, ())?;

    // Expire guardian set
    match GUARDIAN_SETS.may_load(deps.storage, guardian_set_index)? {
        Some(mut gs) => {
            gs.expiration_time = u32::try_from(env.block.time.seconds()).unwrap() + 86400;
            GUARDIAN_SETS.save(dep.storage, guardian_set_index, gs)?;
        }
        None => {}
    };
    GUARDIAN_SETS.save(
        deps.storage,
        upgrade.new_guardian_set_index,
        upgrade.new_guardian_set,
    )?;
    GUARDIAN_SET_INDEX.save(deps.storage, upgrade.new_guardian_set_index)?;
    Ok(())
}

fn verify_governance_vm(vm: &VM) -> Result<()> {
    verify_vm(deps, env, vm)?;

    // only current guardianset can sign governance packets
    ensure!(
        vm.guardian_set_index == GUARDIAN_SET_INDEX.load(deps.storage)?,
        "not signed by current guardian set"
    );

    // verify source
    ensure!(
        vm.emitter_chain_id == governance_chain_id(),
        "wrong governance chain"
    );
    ensure!(
        vm.emitter_address == governance_contract(),
        "wrong governance contract"
    );

    // prevent re-entry
    ensure!(
        !CONSUMED_GOVERNANCE_ACTIONS.has(deps.storage, vm.hash),
        "governance action already consumed"
    );

    Ok(())
}

fn transferOwnership(deps: DepsMut, env: Env, info: MessageInfo, new_owner: Addr) -> Result<()> {
    ensure!(
        OWNER.load(deps.storage) == info.sender,
        "caller is not the owner"
    );
    OWNER.save(deps.storage, info.sender);

    // XXX emit OwnershipTransfered(currentOwner, newOwner);
}
