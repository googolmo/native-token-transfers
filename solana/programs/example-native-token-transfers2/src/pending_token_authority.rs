use borsh::{BorshDeserialize, BorshSerialize};
use shank_macro::ShankAccount;
use solana_pubkey::Pubkey;


#[derive(ShankAccount, BorshDeserialize, BorshSerialize)]
pub struct PendingTokenAuthority {
    pub discriminator: [u8;8],
    pub bump: u8,
    pub pending_authority: Pubkey,
    pub rent_payer: Pubkey,
}

impl PendingTokenAuthority {
    pub const SEED_PREFIX: &'static [u8] = b"pending_token_authority";
}
