use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CompressedProof {
    pub a: [u8; 32],
    pub b: [u8; 64],
    pub c: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Groth16VerifyingKey {
    pub alpha_g1: [u8; 32],
    pub beta_g2: [u8; 64],
    pub gamma_g2: [u8; 64],
    pub delta_g2: [u8; 64],
    pub gamma_abc_g1: Vec<[u8; 32]>,
}

#[account]
pub struct VerifyingKeyAccount {
    pub authority: Pubkey,
    pub circuit_id: String,
    pub key: Groth16VerifyingKey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VerificationContext {
    pub verification_type: String,
    pub public_inputs: Vec<[u8; 32]>,
    pub proof: CompressedProof,
}

#[account]
pub struct VerificationResult {
    pub subject: Pubkey,
    pub verifier: Pubkey,
    pub verification_type: String,
    pub timestamp: i64,
    pub is_valid: bool,
    pub commitment: Option<[u8; 32]>,
    pub expiration: Option<i64>,
}
