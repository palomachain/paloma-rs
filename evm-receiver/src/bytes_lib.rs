use sha3::{Digest, Keccak256};
use std::convert::TryInto;
use std::io;

pub trait ReadBytes: io::Read {
    #[inline]
    fn read_bytes<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl<R: io::Read + ?Sized> ReadBytes for R {}

pub fn double_hash(buf: &[u8]) -> [u8; 32] {
    let mut h = Keccak256::default();
    h.update(buf);
    let buf = h.finalize();
    // XXX(chase): this is present in the original.
    // let buf = ethabi::encode_packed(buf);
    let mut h = Keccak256::default();
    h.update(&buf);
    let buf = h.finalize();
    buf.try_into().unwrap()
}
