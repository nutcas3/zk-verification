use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum CredentialLevel {
    Basic,
    Standard,
    Enhanced,
    Institutional,
}

impl CredentialLevel {
    pub fn level(&self) -> u8 {
        match self {
            CredentialLevel::Basic => 1,
            CredentialLevel::Standard => 2,
            CredentialLevel::Enhanced => 3,
            CredentialLevel::Institutional => 4,
        }
    }
    
    pub fn meets_requirement(&self, required: &CredentialLevel) -> bool {
        self.level() >= required.level()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CredentialMetadata {
    pub level: CredentialLevel,
    pub issuer_authority: Pubkey,
    pub valid_from: i64,
    pub valid_until: i64,
    pub revocation_registry: Pubkey,
    pub jurisdiction: [u8; 2],
}

impl CredentialMetadata {
    pub fn is_valid(&self, current_time: i64) -> bool {
        current_time >= self.valid_from && current_time <= self.valid_until
    }
}

#[account]
pub struct Credential {
    pub owner: Pubkey,
    pub metadata: CredentialMetadata,
    pub commitment: [u8; 32],
    pub is_revoked: bool,
    pub revoked_at: Option<i64>,
    pub bump: u8,
}

impl Credential {
    pub const SIZE: usize = 8 + // discriminator
        32 + // owner
        (1 + 32 + 8 + 8 + 32 + 2) + // metadata
        32 + // commitment
        1 + // is_revoked
        1 + 8 + // revoked_at Option
        1; // bump
}

#[account]
pub struct RevocationRegistry {
    pub authority: Pubkey,
    pub revocation_root: [u8; 32],
    pub total_revoked: u64,
    pub last_updated: i64,
    pub bump: u8,
}

impl RevocationRegistry {
    pub const SIZE: usize = 8 + // discriminator
        32 + // authority
        32 + // revocation_root
        8 + // total_revoked
        8 + // last_updated
        1; // bump
}

#[event]
pub struct CredentialIssued {
    pub credential: Pubkey,
    pub owner: Pubkey,
    pub level: CredentialLevel,
    pub issuer: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct CredentialRevoked {
    pub credential: Pubkey,
    pub owner: Pubkey,
    pub revoked_by: Pubkey,
    pub timestamp: i64,
    pub reason: String,
}
