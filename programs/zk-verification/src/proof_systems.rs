use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ProofSystem {
    Groth16,
    Plonk,
    Stark,
    Halo2,
}

#[account]
pub struct UniversalVerifier {
    pub proof_system: ProofSystem,
    pub authority: Pubkey,
    pub circuit_id: String,
    pub verifying_key: Vec<u8>,
    pub created_at: i64,
    pub is_active: bool,
}

impl UniversalVerifier {
    pub const MAX_VK_SIZE: usize = 2048;
    
    pub const MAX_SIZE: usize = 8 +
        1 +
        32 +
        4 + 100 +
        4 + Self::MAX_VK_SIZE +
        8 +
        1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ProofSystemCapabilities {
    pub requires_trusted_setup: bool,
    pub supports_recursion: bool,
    pub avg_proof_size: u32,
    pub avg_verification_cu: u32,
}

impl ProofSystem {
    pub fn capabilities(&self) -> ProofSystemCapabilities {
        match self {
            ProofSystem::Groth16 => ProofSystemCapabilities {
                requires_trusted_setup: true,
                supports_recursion: false,
                avg_proof_size: 192,
                avg_verification_cu: 50_000,
            },
            ProofSystem::Plonk => ProofSystemCapabilities {
                requires_trusted_setup: true,
                supports_recursion: true,
                avg_proof_size: 512,
                avg_verification_cu: 100_000,
            },
            ProofSystem::Stark => ProofSystemCapabilities {
                requires_trusted_setup: false,
                supports_recursion: true,
                avg_proof_size: 2048,
                avg_verification_cu: 200_000,
            },
            ProofSystem::Halo2 => ProofSystemCapabilities {
                requires_trusted_setup: false,
                supports_recursion: true,
                avg_proof_size: 768,
                avg_verification_cu: 150_000,
            },
        }
    }
}
