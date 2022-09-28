use crate::msg::{Consensus, Valset};
use crate::msg::{LogicCallArgs, Signature};
use crate::Compass;
use anchor_lang::prelude::*;
use std::iter::zip;

/// 2/3 * 2**32.
/// Validator powers will be normalized to sum to 2**32 in every valset update.
const POWER_THRESHOLD: u64 = 2_863_311_530;

#[derive(Accounts)]
pub struct InitData<'info> {
    #[account(mut)]
    pub compass: Account<'info, Compass<'info>>,

    pub smart_contract_id: Pubkey,
    pub valset: Valset,
}

#[derive(Accounts)]
pub struct UpdateValset<'info> {
    #[account(mut)]
    pub compass: Account<'info, Compass<'info>>,

    pub consensus: Consensus,
    pub new_valset: Valset,
}

#[derive(Accounts)]
pub struct SubmitLogicCall<'info> {
    #[account]
    pub compass: Account<'info, Compass<'info>>,

    pub consensus: Consensus,
    pub args: LogicCallArgs,
    pub message_id: Pubkey,
    pub deadline: u64,
}

#[error_code]
pub enum CompassError {
    InsufficientPower,
    InvalidSignature,
    InvalidValsetID,
    RecursiveInvocation,
    ReusedMessageId,
    Timeout,
}

#[program]
mod external_api {
    use super::*;
    use anchor_lang::context::Context;

    pub fn initialize(ctx: Context<InitData<'_>>) -> Result<()> {
        check_validator_power(&ctx.accounts.valset.powers)?;
        ctx.accounts.compass.valset = ctx.accounts.valset.clone();
        ctx.accounts.compass.smart_contract_id = ctx.accounts.smart_contract_id;
        Ok(())
    }

    /// This updates the valset by checking that the validators in the current valset have signed off on the
    /// new valset. The signatures supplied are the signatures of the current valset over the checkpoint hash
    /// generated from the new valset.
    /// Anyone can call this function, but they must supply valid signatures of constant_powerThreshold of the current valset over
    /// the new valset.
    /// valset: new validator set to update with
    /// consensus: current validator set and signatures
    pub fn update_valset(ctx: Context<UpdateValset<'_>>) -> Result<()> {
        // Ensure that the new valset_id is greater than current valset_id.
        require!(
            ctx.accounts.consensus.valset.valset_id < ctx.accounts.new_valset.valset_id,
            CompassError::InvalidValsetID,
        );
        check_validator_power(&ctx.accounts.new_valset.powers)?;
        ctx.accounts.compass.valset = ctx.accounts.new_valset.clone();
        Ok(())
    }

    /// This makes calls to contracts that execute arbitrary logic
    /// message_id is to prevent replay attack and every message_id can be used only once
    pub fn submit_logic_call(ctx: Context<SubmitLogicCall<'_>>) -> Result<()> {
        require!(
            ctx.accounts.args.contract_address != ctx.program_id,
            CompassError::RecursiveInvocation,
        );
        // XXX require!(env.block.time.seconds() < deadline, CompassError::Timeout);
        // XXX
        //require!(
        //    info.funds.iter().all(|coin| coin.amount.is_zero()),
        //    CompassError::FundsShouldBeZero,
        //);
        let message_id_bytes = message_id.to_be_bytes();
        require!(
            !ctx.accounts.compass.contains(&message_id_bytes),
            CompassError::ReusedMessageId,
        );
        // XXX: This is _not_ how to do an arbitrary cross-program invocation in solana.
        //CpiContext::new(
        //    ctx.accounts.program_id.to_account_info(),
        //    &ctx.accounts.args.payload,
        //)
    }
}

fn check_signature(pk: &Pubkey, hash: &[u8; 32], sig: &Signature) -> Result<()> {
    // message: &Message, signature: &Signature, pubkey: &PublicKey
    let b = libsecp256k1::verify(hash, &sig.0, pk);
    require!(b, CompassError::InvalidSignature);
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
    err!(CompassError::InsufficientPower)
}

fn check_validator_signatures(consensus: &Consensus, hash: [u8; 32]) -> Result<()> {
    let mut cumulative_power: u64 = 0;
    for ((validator, power), sig) in
        zip(&consensus.valset.validators, &consensus.valset.powers).zip(&consensus.signatures)
    {
        check_signature(validator, &hash, sig)?;
        cumulative_power += power;
        if cumulative_power >= POWER_THRESHOLD {
            return Ok(());
        }
    }
    err!(CompassError::InsufficientPower)
}
