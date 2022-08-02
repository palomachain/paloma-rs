#![cfg(not(target_arch = "wasm32"))]

use arrayvec::ArrayString;

pub mod contract;
mod state;

fn hex(bytes: &[u8; 32]) -> ArrayString<64> {
    let mut buf = ArrayString::new();
    for b in bytes {
        write!(&mut buf, "{:02x}", b).unwrap();
    }
    buf
}
