use anchor_lang::prelude::*;

#[account]
pub struct ProofNullifier {
    pub nullifier_hash: [u8; 32],
    pub used_at: i64,
    pub verification_type: String,
    pub subject: Pubkey,
    pub bump: u8,
}

impl ProofNullifier {
    pub const MAX_SIZE: usize = 8 +
        32 +
        8 +
        4 + 50 +
        32 +
        1;
}

#[account]
pub struct NullifierRegistry {
    pub authority: Pubkey,
    pub total_nullifiers: u64,
    pub merkle_root: [u8; 32],
    pub bump: u8,
}

impl NullifierRegistry {
    pub const SIZE: usize = 8 +
        32 +
        8 +
        32 +
        1;
}

#[event]
pub struct NullifierUsed {
    pub nullifier_hash: [u8; 32],
    pub verification_type: String,
    pub subject: Pubkey,
    pub timestamp: i64,
}
