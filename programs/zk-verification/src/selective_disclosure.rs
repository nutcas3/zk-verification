use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum AttributeType {
    Age(AgeRange),
    Residency(CountryCode),
    NotSanctioned,
    AccreditedInvestor,
    CreditScore(ScoreRange),
    Employed,
    Income(IncomeRange),
    Custom(String),
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum AgeRange {
    Over18,
    Over21,
    Over25,
    Between18And65,
    Custom { min: u8, max: u8 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct CountryCode {
    pub code: [u8; 2],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ScoreRange {
    Poor,
    Fair,
    Good,
    VeryGood,
    Excellent,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum IncomeRange {
    Under50K,
    Range50To100K,
    Range100To250K,
    Over250K,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SelectiveDisclosureRequest {
    pub required_attributes: Vec<AttributeType>,
    pub optional_attributes: Vec<AttributeType>,
    pub jurisdiction: [u8; 2],
    pub min_credential_level: u8,
}

#[account]
pub struct SelectiveDisclosureProof {
    pub subject: Pubkey,
    pub verifier: Pubkey,
    pub proven_attributes: Vec<AttributeType>,
    pub credential_commitment: [u8; 32],
    pub nullifier: [u8; 32],
    pub timestamp: i64,
    pub expires_at: i64,
    pub is_valid: bool,
}

impl SelectiveDisclosureProof {
    pub const MAX_ATTRIBUTES: usize = 10;
    
    pub const MAX_SIZE: usize = 8 + // discriminator
        32 + // subject
        32 + // verifier
        4 + (Self::MAX_ATTRIBUTES * 100) + // proven_attributes (estimate)
        32 + // credential_commitment
        32 + // nullifier
        8 + // timestamp
        8 + // expires_at
        1; // is_valid
    
    pub fn check_validity(&self, current_time: i64) -> bool {
        self.is_valid && current_time < self.expires_at
    }
}

#[event]
pub struct SelectiveDisclosurePerformed {
    pub subject: Pubkey,
    pub verifier: Pubkey,
    pub attribute_count: u8,
    pub timestamp: i64,
}
