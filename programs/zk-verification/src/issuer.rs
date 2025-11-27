use anchor_lang::prelude::*;

#[account]
pub struct TrustedIssuer {
    pub pubkey: Pubkey,
    pub name: String,
    pub jurisdiction: [u8; 2],
    pub credential_types: Vec<String>,
    pub reputation_score: u32,
    pub total_issued: u64,
    pub revoked_count: u64,
    pub staked_amount: u64,
    pub is_active: bool,
    pub registered_at: i64,
    pub last_activity: i64,
    pub bump: u8,
}

impl TrustedIssuer {
    pub const MAX_NAME_LEN: usize = 100;
    pub const MAX_CREDENTIAL_TYPES: usize = 20;
    pub const MAX_CREDENTIAL_TYPE_LEN: usize = 50;
    
    pub const SIZE: usize = 8 + 
        32 + // pubkey
        4 + Self::MAX_NAME_LEN + // name
        2 + // jurisdiction
        4 + (Self::MAX_CREDENTIAL_TYPES * (4 + Self::MAX_CREDENTIAL_TYPE_LEN)) + // credential_types
        4 + // reputation_score
        8 + // total_issued
        8 + // revoked_count
        8 + // staked_amount
        1 + // is_active
        8 + // registered_at
        8 + // last_activity
        1; // bump
    
    pub fn calculate_reputation(&self) -> u32 {
        if self.total_issued == 0 {
            return 5000;
        }
        
        let revocation_rate = (self.revoked_count as f64) / (self.total_issued as f64);
        let base_score = 10000.0 * (1.0 - revocation_rate);
        
        base_score.max(0.0).min(10000.0) as u32
    }
    
    pub fn can_issue(&self, credential_type: &str) -> bool {
        self.is_active && self.credential_types.iter().any(|t| t == credential_type)
    }
}

#[account]
pub struct IssuerRegistry {
    pub authority: Pubkey,
    pub total_issuers: u64,
    pub active_issuers: u64,
    pub min_stake_amount: u64,
    pub bump: u8,
}

impl IssuerRegistry {
    pub const SIZE: usize = 8 +
        32 +
        8 +
        8 +
        8 +
        1;
}

#[account]
pub struct IssuerStake {
    pub issuer: Pubkey,
    pub amount: u64,
    pub slashing_rules: Vec<SlashingRule>,
    pub total_slashed: u64,
    pub staked_at: i64,
    pub bump: u8,
}

impl IssuerStake {
    pub const MAX_SLASHING_RULES: usize = 10;
    
    pub const SIZE: usize = 8 +
        32 +
        8 +
        4 + (Self::MAX_SLASHING_RULES * 2) +
        8 +
        8 +
        1; // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum SlashingRule {
    FraudulentCredential(u8),
    ExcessiveRevocations(u8),
    SecurityBreach(u8),
}

#[event]
pub struct IssuerRegistered {
    pub issuer: Pubkey,
    pub name: String,
    pub jurisdiction: [u8; 2],
    pub staked_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct IssuerSlashed {
    pub issuer: Pubkey,
    pub reason: String,
    pub amount_slashed: u64,
    pub remaining_stake: u64,
    pub timestamp: i64,
}

#[event]
pub struct IssuerDeactivated {
    pub issuer: Pubkey,
    pub reason: String,
    pub timestamp: i64,
}
