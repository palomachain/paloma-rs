use crate::bytes_lib::ReadBytes;
use crate::structs::GuardianSet;
use byteorder::{BigEndian, ReadBytesExt};
use ethabi::Address;
use eyre::{ensure, Result};
use std::io::{Cursor, Read};

enum GovernanceAction {
    UpgradeContract,
    UpgradeGuardianset,
}

struct GuardianSetUpgrade {
    module: [u8; 32],
    action: u8,
    chain: u16,

    new_guardian_set: GuardianSet,
    new_guardian_set_index: u32,
}

fn parse_guardian_set_upgrade(buf: &[u8]) -> Result<GuardianSetUpgrade> {
    let mut buf = Cursor::new(buf);
    let mut module = [0; 32];
    buf.read_exact(&mut module)?;
    let action = buf.read_u8()?;
    ensure!(action == 2, "invalid GuardianSetUpgrade");
    let chain = buf.read_u16::<BigEndian>()?;
    let new_guardian_set_index = buf.read_u32::<BigEndian>()?;
    let guardian_len = buf.read_u8()?;
    let mut new_guardian_set = GuardianSet {
        keys: Vec::with_capacity(guardian_len.into()),
        expiration_time: 0,
    };

    for _ in 0..guardian_len {
        new_guardian_set
            .keys
            .push(Address::from_slice(&buf.read_bytes::<20>()?));
    }
    ensure!(buf.read_u8().is_err(), "invalid GuardianSetUpgrade");
    Ok(GuardianSetUpgrade {
        module,
        action,
        chain,
        new_guardian_set,
        new_guardian_set_index,
    })
}
