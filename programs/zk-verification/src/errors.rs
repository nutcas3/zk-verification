use anchor_lang::prelude::*;
use thiserror::Error;

#[error_code]
pub enum ZkVerificationError {
    #[msg("Public inputs conversion failed")]
    PublicInputsTryIntoFailed,
    
    #[msg("Failed to decompress G1 point")]
    DecompressG1Failed,
    
    #[msg("Failed to decompress G2 point")]
    DecompressG2Failed,
    
    #[msg("Invalid public inputs length")]
    InvalidPublicInputsLength,
    
    #[msg("Failed to create Groth16 verifier")]
    CreateGroth16VerifierFailed,
    
    #[msg("Proof verification failed")]
    ProofVerificationFailed,
    
    #[msg("Invalid batch size")]
    InvalidBatchSize,
    
    #[msg("Invalid proof format")]
    InvalidProofFormat,
    
    #[msg("Nullifier already used - replay attack prevented")]
    NullifierAlreadyUsed,
    
    #[msg("Credential has been revoked")]
    CredentialRevoked,
    
    #[msg("Credential has expired")]
    CredentialExpired,
    
    #[msg("Insufficient credential level")]
    InsufficientCredentialLevel,
    
    #[msg("Issuer not authorized")]
    IssuerNotAuthorized,
    
    #[msg("Issuer is inactive")]
    IssuerInactive,
    
    #[msg("Insufficient stake amount")]
    InsufficientStake,
    
    #[msg("Invalid jurisdiction")]
    InvalidJurisdiction,
    
    #[msg("Attribute not proven")]
    AttributeNotProven,
    
    #[msg("Disclosure proof expired")]
    DisclosureProofExpired,
}
