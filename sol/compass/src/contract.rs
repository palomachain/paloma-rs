use crate::state::{LAST_CHECKPOINT, TURNSTONE_ID};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use ethabi::ethereum_types::U256;
use ethabi::Address;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, msg};
use solana_sdk::{keccak, timing};
use std::iter::zip;

const MAX_VALIDATORS: usize = 320;

/// 2/3 * 2**32.
/// Validator powers will be normalized to sum to 2**32 in every valset update.
const POWER_THRESHOLD: usize = 2_863_311_530;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct Valset {
    pub validators: Vec<Address>,
    pub powers: Vec<U256>,
    pub valset_id: U256,
}

impl Valset {
    /// Make a new checkpoint from the supplied validator set.
    ///
    /// A checkpoint is a hash of all relevant information about the valset. This is stored by the contract,
    /// instead of storing the information directly. This saves on storage and gas.
    ///
    /// The format of the checkpoint is:
    /// ```
    /// keccak256(abi_encode(checkpoint(validators, powers, valset_id, turnstone_id)))
    /// ```
    /// The validator powers must be decreasing or equal. This is important for checking the signatures on the
    /// next valset, since it allows the caller to stop verifying signatures once a quorum of signatures have been verified.
    fn make_checkpoint(&self) -> [u8; 32] {
        use ethabi::Token;
        keccak::hash(ethabi::encode(&[
            Token::Array(self.validators.map(|a| Token::Address(a)).collect()),
            //&self.powers,
            //&self.valset_id,
            //turnstone_id(),
        ]))
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
struct Signature {
    v: U256,
    r: U256,
    s: U256,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
struct Consensus {
    valset: Valset,
    /// Signatures must be in the same order as the validator array in `valset`
    signatures: Vec<Signature>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
struct LogicCallArgs {
    /// The arbitrary contract address to external call.
    logic_contract_address: Pubkey,
    payload: Vec<u8>,
}

// Event
#[derive(Debug)]
struct ValsetUpdated {
    checkpoint: [u8; 32],
    valset_id: U256,
}

// Event
#[derive(Debug)]
struct LogicCallEvent {
    logic_contract_address: Pubkey,
    payload: Vec<u8>,
    message_id: U256,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
enum Instruction {
    Init([u8; 32]),
    TurnstoneId,
    UpdateValset(Consensus, Valset),
    SubmitLogicCall(Consensus, LogicCallArgs, U256, U256),
}

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    use Instruction::*;
    let inst = Instruction::try_from(instruction_data)?;
    match inst {
        Init(turnstone_id) => init(accounts, turnstone_id),
        TurnstoneId => turnstone_id(accounts),
        UpdateValset(consensus, new_valset) => update_valset(&consensus, &new_valset),
        SubmitLogicCall(consensus, args, message_id, deadline) => {
            submit_logic_call(&consensus, &args, message_id, deadline)
        }
    }
}

// entrypoint
pub fn init(accounts: &[AccountInfo], turnstone_id: [u8; 32], valset: Valset) -> ProgramResult {
    TURNSTONE_ID.store(&accounts[0], &turnstone_id)?;
    check_validator_power(&valset.powers);
    let new_checkpoint = valset.make_checkpoint();
    LAST_CHECKPOINT.store(&new_checkpoint);
    msg!(
        "{:?}",
        ValsetUpdated {
            checkpoint: new_checkpoint,
            valset_id: valset.valset_id
        }
    );
    Ok(())
}

// entrypoint
pub fn turnstone_id(accounts: &[AccountInfo]) -> ProgramResult {
    msg!(&hex(TURNSTONE_ID.load(&accounts[0])));
    Ok(())
}

fn check_eip712_signature(signer: &Address, hash: [u8; 32], sig: &Signature) {
    //    message_digest: bytes32 = keccak256(concat(convert("\x19Ethereum Signed Message:\n32", Bytes[28]), hash))
    assert_eq!(
        signer,
        ecrecover(message_digest, sig.v, sig.r, sig.s),
        "Invalid Signature"
    );
}

fn check_validator_power(powers: &[U256]) {
    let mut cumulative_power = 0;
    for power in &powers {
        cumulative_power += power;
        if cumulative_power >= POWER_THRESHOLD {
            return;
        }
    }
    assert!(cumulative_power >= POWER_THRESHOLD, "Insufficient Power");
}

fn check_validator_signatures(consensus: &Consensus, hash: [u8; 32]) {
    let mut cumulative_power = 0;
    for ((validator, power), sig) in
        zip(&consensus.valset.validators, &consensus.valset.powers).zip(&consensus.signatures)
    {
        if sig.v != 0 {
            check_eip712_signature(validator, hash, sig);
            cumulative_power += power;
            if cumulative_power >= POWER_THRESHOLD {
                break;
            }
        }
    }
    assert!(cumulative_power >= POWER_THRESHOLD, "Insufficient Power");
}

/// This updates the valset by checking that the validators in the current valset have signed off on the
/// new valset. The signatures supplied are the signatures of the current valset over the checkpoint hash
/// generated from the new valset.
/// Anyone can call this function, but they must supply valid signatures of constant_powerThreshold of the current valset over
/// the new valset.
/// valset: new validator set to update with
/// consensus: current validator set and signatures
// entrypoint
fn update_valset(consensus: &Consensus, new_valset: &Valset) -> ProgramResult {
    // check if new valset_id is greater than current valset_id
    assert!(
        new_valset.valset_id > consensus.valset.valset_id,
        "Invalid Valset ID"
    );
    check_validator_power(&new_valset.powers);
    // check if the supplied current validator set matches the saved checkpoint
    assert_eq!(
        LAST_CHECKPOINT.load(todo!()),
        consensus.valset.make_checkpoint(),
        "Incorrect Checkpoint"
    );
    // calculate the new checkpoint
    let new_checkpoint: bytes32 = new_valset.make_checkpoint();
    // check if enough validators signed new validator set (new checkpoint)
    check_validator_signatures(consensus, new_checkpoint);
    LAST_CHECKPOINT.store(todo!(), new_checkpoint);
    msg!("{:?}", ValsetUpdated(new_checkpoint, new_valset.valset_id));
    Ok(())
}

/// This makes calls to contracts that execute arbitrary logic
/// message_id is to prevent replay attack and every message_id can be used only once
// entrypoint
fn submit_logic_call(
    consensus: &Consensus,
    args: &LogicCallArgs,
    message_id: U256,
    deadline: U256,
) {
    assert!(timing::timestamp() <= deadline, "Timeout");
    assert!(!self.message_id_used[message_id], "Used Message_ID");
    self.message_id_used[message_id] = true;
    // check if the supplied current validator set matches the saved checkpoint
    assert_eq!(
        LAST_CHECKPOINT.load(todo!()),
        consensus.valset.make_checkpoint(),
        "Incorrect Checkpoint"
    );
    // signing data is keccak256 hash of abi_encoded logic_call(args, message_id, turnstone_id, deadline)
    let args_hash = keccak::hash(&ethabi::encode(&[
        args,
        message_id,
        TURNSTONE_ID.load(todo!()),
        deadline,
    ]));
    // check if enough validators signed args_hash
    self.check_validator_signatures(consensus, args_hash);
    // make call to logic contract
    raw_call(args.logic_contract_address, args.payload);
    msg!(
        "{:?}",
        LogicCallEvent(args.logic_contract_address, args.payload, message_id)
    );
}
