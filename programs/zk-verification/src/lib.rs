use anchor_lang::prelude::*;

mod errors;
mod proof;
mod verifier;
#[cfg(test)]
mod tests;

use errors::ZkVerificationError;
use proof::{CompressedProof, Groth16VerifyingKey, VerificationContext, VerificationResult, VerifyingKeyAccount};
use verifier::verify;

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
    
    /// Verify a ZK proof
    pub fn verify_proof(
        ctx: Context<VerifyProof>,
        verification_context: VerificationContext,
    ) -> Result<()> {
        let vk = &ctx.accounts.verifying_key.key;
        let proof = &verification_context.proof;
        
        // Convert public inputs to fixed-size array (simplified for demo)
        // In a real implementation, we'd handle different sizes properly
        if verification_context.public_inputs.len() != 2 {
            return Err(ZkVerificationError::InvalidPublicInputsLength.into());
        }
        
        let public_inputs: [[u8; 32]; 2] = [
            verification_context.public_inputs[0],
            verification_context.public_inputs[1],
        ];
        
        // Verify the proof
        verify(&public_inputs, proof, vk)?;
        
        // Record verification result
        let result = &mut ctx.accounts.verification_result;
        result.subject = ctx.accounts.subject.key();
        result.verifier = ctx.accounts.authority.key();
        result.verification_type = verification_context.verification_type;
        result.timestamp = Clock::get()?.unix_timestamp;
        result.is_valid = true;
        
        // Optional: store a commitment hash if provided
        if !public_inputs[0].iter().all(|&b| b == 0) {
            result.commitment = Some(public_inputs[0]);
        }
        
        msg!("Proof verified successfully for subject: {}", result.subject);
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
    
    pub system_program: Program<'info, System>,
}
