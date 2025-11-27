use anchor_lang::prelude::*;

mod errors;
mod proof;
mod verifier;
mod nullifier;
mod proof_systems;
mod credentials;
mod selective_disclosure;
mod issuer;
mod public_inputs;
#[cfg(test)]
mod tests;

use errors::ZkVerificationError;
use proof::{CompressedProof, Groth16VerifyingKey, VerificationContext, VerificationResult, VerifyingKeyAccount};
use verifier::{verify, verify_dynamic};
use public_inputs::PublicInputsHandler;
use nullifier::{ProofNullifier, NullifierRegistry, NullifierUsed};
use proof_systems::{ProofSystem, UniversalVerifier};
use credentials::{Credential, CredentialLevel, CredentialMetadata, RevocationRegistry, CredentialIssued, CredentialRevoked};
use selective_disclosure::{SelectiveDisclosureProof, SelectiveDisclosureRequest, AttributeType, SelectiveDisclosurePerformed};
use issuer::{TrustedIssuer, IssuerRegistry, IssuerStake, IssuerRegistered, IssuerSlashed, IssuerDeactivated};

declare_id!("4Whhcd4H1ud4RgeJV7uczjyWmTRiBHt1ioKiu9bEFYAX");

#[program]
pub mod zk_verification {
    use super::*;

    /// Initialize the program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("ZK Verification program initialized: {:?}", ctx.program_id);
        Ok(())
    }
    
    /// Register a new verifying key for a specific circuit
    pub fn register_verifying_key(
        ctx: Context<RegisterVerifyingKey>,
        circuit_id: String,
        key: Groth16VerifyingKey,
    ) -> Result<()> {
        let vk_account = &mut ctx.accounts.verifying_key;
        vk_account.authority = ctx.accounts.authority.key();
        vk_account.circuit_id = circuit_id;
        vk_account.key = key;
        
        msg!("Registered verifying key for circuit: {}", circuit_id);
        Ok(())
    }
    
    /// Verify a ZK proof with nullifier to prevent replay attacks
    pub fn verify_proof(
        ctx: Context<VerifyProof>,
        verification_context: VerificationContext,
        nullifier_hash: [u8; 32],
    ) -> Result<()> {
        let vk = &ctx.accounts.verifying_key.key;
        let proof = &verification_context.proof;
        let current_time = Clock::get()?.unix_timestamp;
        
        // Use PublicInputsHandler for variable-sized inputs
        let inputs_handler = PublicInputsHandler::new(verification_context.public_inputs.clone())?;
        
        // Validate inputs match the verifying key's expected count
        inputs_handler.validate_against_vk(vk.gamma_abc_g1.len())?;
        
        msg!(
            "Verifying proof with {} public inputs for verification type: {}",
            inputs_handler.len(),
            verification_context.verification_type
        );
        
        // Verify the proof using dynamic verification
        verify_dynamic(inputs_handler.as_slice(), proof, vk)?;
        
        // Initialize nullifier to prevent replay
        let nullifier = &mut ctx.accounts.nullifier;
        nullifier.nullifier_hash = nullifier_hash;
        nullifier.used_at = current_time;
        nullifier.verification_type = verification_context.verification_type.clone();
        nullifier.subject = ctx.accounts.subject.key();
        nullifier.bump = ctx.bumps.nullifier;
        
        // Update nullifier registry
        let registry = &mut ctx.accounts.nullifier_registry;
        registry.total_nullifiers = registry.total_nullifiers.checked_add(1).unwrap();
        
        // Record verification result
        let result = &mut ctx.accounts.verification_result;
        result.subject = ctx.accounts.subject.key();
        result.verifier = ctx.accounts.authority.key();
        result.verification_type = verification_context.verification_type.clone();
        result.timestamp = current_time;
        result.is_valid = true;
        
        // Extract and store commitment from first public input (if not all zeros)
        if let Some(commitment) = inputs_handler.extract_commitment() {
            if !commitment.iter().all(|&b| b == 0) {
                result.commitment = Some(commitment);
            }
        }
        
        // Emit event
        emit!(NullifierUsed {
            nullifier_hash,
            verification_type: verification_context.verification_type,
            subject: ctx.accounts.subject.key(),
            timestamp: current_time,
        });
        
        msg!("Proof verified successfully for subject: {}", result.subject);
        Ok(())
    }
    
    /// Initialize nullifier registry
    pub fn initialize_nullifier_registry(ctx: Context<InitializeNullifierRegistry>) -> Result<()> {
        let registry = &mut ctx.accounts.nullifier_registry;
        registry.authority = ctx.accounts.authority.key();
        registry.total_nullifiers = 0;
        registry.merkle_root = [0; 32];
        registry.bump = ctx.bumps.nullifier_registry;
        
        msg!("Nullifier registry initialized");
        Ok(())
    }
    
    /// Register a universal verifier for any proof system
    pub fn register_universal_verifier(
        ctx: Context<RegisterUniversalVerifier>,
        proof_system: ProofSystem,
        circuit_id: String,
        verifying_key: Vec<u8>,
    ) -> Result<()> {
        let verifier = &mut ctx.accounts.universal_verifier;
        verifier.proof_system = proof_system;
        verifier.authority = ctx.accounts.authority.key();
        verifier.circuit_id = circuit_id;
        verifier.verifying_key = verifying_key;
        verifier.created_at = Clock::get()?.unix_timestamp;
        verifier.is_active = true;
        
        msg!("Universal verifier registered for proof system: {:?}", verifier.proof_system);
        Ok(())
    }
    
    /// Issue a credential to a user
    pub fn issue_credential(
        ctx: Context<IssueCredential>,
        metadata: CredentialMetadata,
        commitment: [u8; 32],
    ) -> Result<()> {
        // Verify issuer is authorized
        require!(
            ctx.accounts.issuer.is_active,
            ZkVerificationError::InvalidProofFormat
        );
        
        let credential = &mut ctx.accounts.credential;
        credential.owner = ctx.accounts.owner.key();
        credential.metadata = metadata.clone();
        credential.commitment = commitment;
        credential.is_revoked = false;
        credential.revoked_at = None;
        credential.bump = ctx.bumps.credential;
        
        // Update issuer stats
        let issuer = &mut ctx.accounts.issuer;
        issuer.total_issued = issuer.total_issued.checked_add(1).unwrap();
        issuer.last_activity = Clock::get()?.unix_timestamp;
        
        // Emit event
        emit!(CredentialIssued {
            credential: ctx.accounts.credential.key(),
            owner: credential.owner,
            level: metadata.level,
            issuer: issuer.pubkey,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Credential issued to: {}", credential.owner);
        Ok(())
    }
    
    /// Revoke a credential
    pub fn revoke_credential(
        ctx: Context<RevokeCredential>,
        reason: String,
    ) -> Result<()> {
        let credential = &mut ctx.accounts.credential;
        let current_time = Clock::get()?.unix_timestamp;
        
        credential.is_revoked = true;
        credential.revoked_at = Some(current_time);
        
        // Update revocation registry
        let registry = &mut ctx.accounts.revocation_registry;
        registry.total_revoked = registry.total_revoked.checked_add(1).unwrap();
        registry.last_updated = current_time;
        
        // Update issuer stats
        let issuer = &mut ctx.accounts.issuer;
        issuer.revoked_count = issuer.revoked_count.checked_add(1).unwrap();
        issuer.reputation_score = issuer.calculate_reputation();
        
        // Emit event
        emit!(CredentialRevoked {
            credential: ctx.accounts.credential.key(),
            owner: credential.owner,
            revoked_by: ctx.accounts.authority.key(),
            timestamp: current_time,
            reason,
        });
        
        msg!("Credential revoked: {}", ctx.accounts.credential.key());
        Ok(())
    }
    
    /// Register a trusted issuer
    pub fn register_issuer(
        ctx: Context<RegisterIssuer>,
        name: String,
        jurisdiction: [u8; 2],
        credential_types: Vec<String>,
        stake_amount: u64,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.issuer_registry;
        let issuer = &mut ctx.accounts.issuer;
        let stake = &mut ctx.accounts.issuer_stake;
        let current_time = Clock::get()?.unix_timestamp;
        
        // Initialize issuer
        issuer.pubkey = ctx.accounts.issuer_authority.key();
        issuer.name = name.clone();
        issuer.jurisdiction = jurisdiction;
        issuer.credential_types = credential_types;
        issuer.reputation_score = 5000; // Start with neutral score
        issuer.total_issued = 0;
        issuer.revoked_count = 0;
        issuer.staked_amount = stake_amount;
        issuer.is_active = true;
        issuer.registered_at = current_time;
        issuer.last_activity = current_time;
        issuer.bump = ctx.bumps.issuer;
        
        // Initialize stake
        stake.issuer = issuer.pubkey;
        stake.amount = stake_amount;
        stake.slashing_rules = vec![];
        stake.total_slashed = 0;
        stake.staked_at = current_time;
        stake.bump = ctx.bumps.issuer_stake;
        
        // Update registry
        registry.total_issuers = registry.total_issuers.checked_add(1).unwrap();
        registry.active_issuers = registry.active_issuers.checked_add(1).unwrap();
        
        // Emit event
        emit!(IssuerRegistered {
            issuer: issuer.pubkey,
            name,
            jurisdiction,
            staked_amount: stake_amount,
            timestamp: current_time,
        });
        
        msg!("Issuer registered: {}", issuer.pubkey);
        Ok(())
    }
    
    /// Perform selective disclosure
    pub fn selective_disclosure(
        ctx: Context<SelectiveDisclosure>,
        request: SelectiveDisclosureRequest,
        nullifier: [u8; 32],
        expires_in: i64,
    ) -> Result<()> {
        let credential = &ctx.accounts.credential;
        let current_time = Clock::get()?.unix_timestamp;
        
        // Verify credential is valid and not revoked
        require!(!credential.is_revoked, ZkVerificationError::InvalidProofFormat);
        require!(
            credential.metadata.is_valid(current_time),
            ZkVerificationError::InvalidProofFormat
        );
        
        // Verify credential level meets requirement
        require!(
            credential.metadata.level.level() >= request.min_credential_level,
            ZkVerificationError::InvalidProofFormat
        );
        
        let proof = &mut ctx.accounts.disclosure_proof;
        proof.subject = ctx.accounts.subject.key();
        proof.verifier = ctx.accounts.verifier.key();
        proof.proven_attributes = request.required_attributes;
        proof.credential_commitment = credential.commitment;
        proof.nullifier = nullifier;
        proof.timestamp = current_time;
        proof.expires_at = current_time + expires_in;
        proof.is_valid = true;
        
        // Emit event
        emit!(SelectiveDisclosurePerformed {
            subject: proof.subject,
            verifier: proof.verifier,
            attribute_count: proof.proven_attributes.len() as u8,
            timestamp: current_time,
        });
        
        msg!("Selective disclosure performed for: {}", proof.subject);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct RegisterVerifyingKey<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(init, payer = authority, space = 8 + 32 + 100 + 500)]
    pub verifying_key: Account<'info, VerifyingKeyAccount>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyProof<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// The subject of the verification (e.g., user being KYC'd)
    /// May or may not be a signer depending on the use case
    pub subject: AccountInfo<'info>,
    
    /// The verifying key to use
    #[account(has_one = authority @ ZkVerificationError::InvalidProofFormat)]
    pub verifying_key: Account<'info, VerifyingKeyAccount>,
    
    /// Account to store the verification result
    #[account(init, payer = authority, space = 8 + 32 + 32 + 100 + 8 + 1 + 33 + 9)]
    pub verification_result: Account<'info, VerificationResult>,
    
    /// Nullifier to prevent replay attacks
    #[account(
        init,
        payer = authority,
        space = ProofNullifier::MAX_SIZE,
        seeds = [b"nullifier", &nullifier_hash],
        bump
    )]
    pub nullifier: Account<'info, ProofNullifier>,
    
    /// Nullifier registry
    #[account(
        mut,
        seeds = [b"nullifier_registry"],
        bump = nullifier_registry.bump
    )]
    pub nullifier_registry: Account<'info, NullifierRegistry>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeNullifierRegistry<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = NullifierRegistry::SIZE,
        seeds = [b"nullifier_registry"],
        bump
    )]
    pub nullifier_registry: Account<'info, NullifierRegistry>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterUniversalVerifier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(init, payer = authority, space = UniversalVerifier::MAX_SIZE)]
    pub universal_verifier: Account<'info, UniversalVerifier>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssueCredential<'info> {
    #[account(mut)]
    pub issuer_authority: Signer<'info>,
    
    /// Owner of the credential
    pub owner: AccountInfo<'info>,
    
    /// Issuer account
    #[account(
        mut,
        seeds = [b"issuer", issuer_authority.key().as_ref()],
        bump = issuer.bump
    )]
    pub issuer: Account<'info, TrustedIssuer>,
    
    /// Credential account
    #[account(
        init,
        payer = issuer_authority,
        space = Credential::SIZE,
        seeds = [b"credential", owner.key().as_ref(), issuer.pubkey.as_ref()],
        bump
    )]
    pub credential: Account<'info, Credential>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeCredential<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Credential to revoke
    #[account(mut)]
    pub credential: Account<'info, Credential>,
    
    /// Revocation registry
    #[account(
        mut,
        seeds = [b"revocation_registry", credential.metadata.issuer_authority.as_ref()],
        bump = revocation_registry.bump
    )]
    pub revocation_registry: Account<'info, RevocationRegistry>,
    
    /// Issuer account
    #[account(
        mut,
        seeds = [b"issuer", credential.metadata.issuer_authority.as_ref()],
        bump = issuer.bump
    )]
    pub issuer: Account<'info, TrustedIssuer>,
}

#[derive(Accounts)]
pub struct RegisterIssuer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Issuer authority (the one who will issue credentials)
    pub issuer_authority: Signer<'info>,
    
    /// Issuer registry
    #[account(
        mut,
        seeds = [b"issuer_registry"],
        bump = issuer_registry.bump
    )]
    pub issuer_registry: Account<'info, IssuerRegistry>,
    
    /// Issuer account
    #[account(
        init,
        payer = authority,
        space = TrustedIssuer::SIZE,
        seeds = [b"issuer", issuer_authority.key().as_ref()],
        bump
    )]
    pub issuer: Account<'info, TrustedIssuer>,
    
    /// Issuer stake account
    #[account(
        init,
        payer = authority,
        space = IssuerStake::SIZE,
        seeds = [b"issuer_stake", issuer_authority.key().as_ref()],
        bump
    )]
    pub issuer_stake: Account<'info, IssuerStake>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SelectiveDisclosure<'info> {
    #[account(mut)]
    pub subject: Signer<'info>,
    
    /// Verifier requesting the disclosure
    pub verifier: AccountInfo<'info>,
    
    /// Subject's credential
    #[account(
        seeds = [b"credential", subject.key().as_ref()],
        bump = credential.bump
    )]
    pub credential: Account<'info, Credential>,
    
    /// Disclosure proof account
    #[account(
        init,
        payer = subject,
        space = SelectiveDisclosureProof::MAX_SIZE
    )]
    pub disclosure_proof: Account<'info, SelectiveDisclosureProof>,
    
    pub system_program: Program<'info, System>,
}
