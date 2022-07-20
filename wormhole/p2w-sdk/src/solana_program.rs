///! Reimplement a small subset of solana_program to avoid bringing in that dependency.

pub mod clock {
    pub type UnixTimestamp = i64;
}

pub mod pubkey {
    #[repr(transparent)]
    #[derive(
        Clone,
        Copy,
        Debug,
        Default,
        serde::Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde::Serialize,
    )]
    pub struct Pubkey([u8; 32]);

    impl Pubkey {
        pub fn new(pubkey_vec: &[u8]) -> Self {
            Self(
                <[u8; 32]>::try_from(<&[u8]>::clone(&pubkey_vec))
                    .expect("Slice must be the same length as a Pubkey"),
            )
        }

        pub fn new_from_array(pubkey_vec: [u8; 32]) -> Self {
            Self(pubkey_vec)
        }

        pub fn to_bytes(self) -> [u8; 32] {
            self.0
        }
    }
}
