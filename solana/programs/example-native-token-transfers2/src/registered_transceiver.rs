use borsh::{BorshDeserialize, BorshSerialize};
use shank_macro::ShankAccount;
use solana_pubkey::Pubkey;


#[derive(ShankAccount, BorshDeserialize, BorshSerialize)]
pub struct RegisteredTransceiver {
    pub discriminator: [u8; 8],
    pub bump: u8,
    pub id: u8,
    pub transceiver_address: Pubkey,
}

impl RegisteredTransceiver {
    pub const SEED_PREFIX: &'static [u8] = b"registered_transceiver";
}
