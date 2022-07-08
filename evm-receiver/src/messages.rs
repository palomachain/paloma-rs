use crate::bytes_lib::{double_hash, ReadBytes};
use crate::state::{GUARDIAN_SETS, GUARDIAN_SET_INDEX};
use crate::structs::{GuardianSet, Signature, VM};
use byteorder::{BigEndian, ReadBytesExt};
use cosmwasm_std::{Deps, Env};
use eyre::{bail, ensure, Result};
use std::convert::TryInto;
use std::io::Cursor;

/// @dev parseAndVerifyVM serves to parse an encodedVM and wholy validate it for consumption
fn parse_and_verify_vm(deps: Deps, env: Env, encoded_vm: &[u8]) -> Result<VM> {
    let vm = parse_vm(encoded_vm)?;
    verify_vm(deps, env, &vm)?;
    Ok(vm)
}

/// @dev `verifyVM` serves to validate an arbitrary vm against a valid Guardian set
///  - it aims to make sure the VM is for a known guardian_set
///  - it aims to ensure the guardian_set is not expired
///  - it aims to ensure the VM has reached quorum
///  - it aims to verify the signatures provided against the guardian_set
///
pub fn verify_vm(deps: Deps, env: Env, vm: &VM) -> Result<()> {
    // @dev Obtain the current GuardianSet for the guardian_set_index provided
    let guardian_set = GUARDIAN_SETS.load(deps.storage, vm.guardian_set_index)?;

    //@dev Checks whether the GuardianSet has zero keys
    //WARNING: This keys check is critical to ensure the GuardianSet has keys present AND to ensure
    //that guardian set key size doesn't fall to zero and negatively impact quorum assessment.  If guardian set
    //key length is 0 and vm.signatures length is 0, this could compromise the integrity of both vm and
    //signature verification.
    //
    if guardian_set.keys.is_empty() {
        bail!("invalid guardian set");
    }

    // @dev Checks if VM guardian set index matches the current index (unless the current set is expired).
    if vm.guardian_set_index != GUARDIAN_SET_INDEX.load(deps.storage)?
        && u64::from(guardian_set.expiration_time) < env.block.time.seconds()
    {
        bail!("guardian set has expired");
    }

    //@dev We're using a fixed point number transformation with 1 decimal to deal with rounding.
    //  WARNING: This quorum check is critical to assessing whether we have enough Guardian signatures to validate a VM
    //  if making any changes to this, obtain additional peer review. If guardian_set key length is 0 and
    //  vm.signatures length is 0, this could compromise the integrity of both vm and signature verification.
    //
    if ((guardian_set.keys.len() * 10 / 3) * 2) / 10 + 1 > vm.signatures.len() {
        bail!("no quorum");
    }

    // @dev Verify the proposed vm.signatures against the guardian_set
    verify_signatures(vm.hash, &vm.signatures, &guardian_set)?;

    // If we are here, we've validated the VM is a valid multi-sig that matches the guardian_set.
    Ok(())
}

/// @dev verifySignatures serves to validate arbitrary sigatures against an arbitrary guardian_set
///  - it intentionally does not solve for expectations within guardian_set (you should use verifyVM if you need these protections)
///  - it intentioanlly does not solve for quorum (you should use verifyVM if you need these protections)
///  - it intentionally returns true when signatures is an empty set (you should use verifyVM if you need these protections)
fn verify_signatures(
    hash: [u8; 32],
    signatures: &[Signature],
    guardian_set: &GuardianSet,
) -> Result<()> {
    ensure!(
        signatures
            .iter()
            .map(|sig| sig.guardian_index)
            .zip(signatures.iter().map(|sig| sig.guardian_index).skip(1))
            .all(|(a, b)| a < b),
        "signature indices must be ascending"
    );
    for sig in signatures {
        // Check to see if the signer of the signature does not match a specific Guardian key at the provided index.
        if recover(hash, sig.v, sig.r, sig.s) != guardian_set.keys[sig.guardian_index.into()] {
            bail!("VM signature invalid");
        }
    }
    // We have validated that the provided signatures are valid for the provided guardian_set
    Ok(())
}

/// @dev parseVM serves to parse an encodedVM into a vm struct
///  - it intentionally performs no validation functions, it simply parses raw into a struct
pub fn parse_vm(buf: &[u8]) -> Result<VM> {
    let mut buf = Cursor::new(buf);

    let version = buf.read_u8()?;
    ensure!(version == 1, "VM version incompatible");

    let guardian_set_index = buf.read_u32::<BigEndian>()?;

    // Parse Signatures
    let signers_len = buf.read_u8()?;
    let mut signatures = Vec::with_capacity(signers_len.into());
    for _ in 0..signers_len {
        signatures.push(Signature {
            guardian_index: buf.read_u8()?,
            r: buf.read_bytes::<32>()?,
            s: buf.read_bytes::<32>()?,
            v: buf.read_u8()?,
        });
    }

    // Hash the body
    let buf = buf.into_inner();
    let hash = double_hash(buf);

    // Parse the body
    let mut buf = Cursor::new(buf);
    let timestamp = buf.read_u32::<BigEndian>()?;
    let nonce = buf.read_u32::<BigEndian>()?;
    let emitter_chain_id = buf.read_u16::<BigEndian>()?;
    let emitter_address = buf.read_bytes::<32>()?;
    let sequence = buf.read_u64::<BigEndian>()?;
    let consistency_level = buf.read_u8()?;

    let payload = Vec::from(buf.into_inner());
    Ok(VM {
        version,
        timestamp,
        nonce,
        emitter_chain_id,
        emitter_address,
        sequence,
        consistency_level,
        payload,
        guardian_set_index,
        signatures,
        hash: hash.try_into().unwrap(),
    })
}
