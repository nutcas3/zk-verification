use crate::errors::ZkVerificationError;
use crate::proof::{CompressedProof, Groth16VerifyingKey};
use anchor_lang::prelude::*;
use ark_bn254::{Bn254, Fq, Fq2, G1Affine, G2Affine};
use ark_ec::{bn::BnParameters, AffineCurve, PairingEngine};
use ark_ff::{BigInteger, Fp2, QuadExtField};
use ark_serialize::{CanonicalDeserialize, Compress, Flags};

/// Decompress a G1 point from compressed bytes
pub fn decompress_g1(compressed: &[u8; 32]) -> Result<G1Affine> {
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(compressed);
    
    // Set compression flag
    bytes[0] |= 0x80;
    
    G1Affine::deserialize_with_mode(&bytes[..], Compress::Yes, Validate::No)
        .map_err(|_| ZkVerificationError::DecompressG1Failed.into())
}

/// Decompress a G2 point from compressed bytes
pub fn decompress_g2(compressed: &[u8; 64]) -> Result<G2Affine> {
    let mut c1 = [0u8; 32];
    c1.copy_from_slice(&compressed[0..32]);
    
    let mut c2 = [0u8; 32];
    c2.copy_from_slice(&compressed[32..64]);
    
    // Set compression flag
    c1[0] |= 0x80;
    
    let mut bytes = Vec::with_capacity(64);
    bytes.extend_from_slice(&c1);
    bytes.extend_from_slice(&c2);
    
    G2Affine::deserialize_with_mode(&bytes[..], Compress::Yes, Validate::No)
        .map_err(|_| ZkVerificationError::DecompressG2Failed.into())
}

/// Groth16 verifier for BN254 curve
pub struct Groth16Verifier<'a> {
    proof_a: &'a G1Affine,
    proof_b: &'a G2Affine,
    proof_c: &'a G1Affine,
    public_inputs: &'a [[u8; 32]],
    vk: &'a Groth16VerifyingKey,
}

impl<'a> Groth16Verifier<'a> {
    /// Create a new Groth16 verifier
    pub fn new(
        proof_a: &'a G1Affine,
        proof_b: &'a G2Affine,
        proof_c: &'a G1Affine,
        public_inputs: &'a [[u8; 32]],
        vk: &'a Groth16VerifyingKey,
    ) -> Result<Self> {
        // Validate inputs
        if public_inputs.len() != vk.gamma_abc_g1.len() - 1 {
            return Err(ZkVerificationError::InvalidPublicInputsLength.into());
        }
        
        Ok(Self {
            proof_a,
            proof_b,
            proof_c,
            public_inputs,
            vk,
        })
    }
    
    /// Verify the proof
    pub fn verify(&self) -> Result<()> {
        // This is a simplified placeholder for the actual verification logic
        // In a real implementation, this would perform pairing checks
        
        // For now, we'll just log that verification was attempted
        msg!("Verifying proof with {} public inputs", self.public_inputs.len());
        
        // In a real implementation, we would:
        // 1. Deserialize the verifying key components
        // 2. Compute linear combination of public inputs with gamma_abc_g1
        // 3. Perform pairing checks: e(A,B) = e(alpha,beta) * e(L,gamma) * e(C,delta)
        
        // For now, return success (this should be replaced with actual verification)
        Ok(())
    }
}

/// Verify a proof with the given public inputs and verifying key (fixed-size)
pub fn verify<const N: usize>(
    public_inputs: &[[u8; 32]; N],
    proof: &CompressedProof,
    vk: &Groth16VerifyingKey,
) -> Result<()> {
    // Decompress proof points
    let proof_a = decompress_g1(&proof.a)?;
    let proof_b = decompress_g2(&proof.b)?;
    let proof_c = decompress_g1(&proof.c)?;
    
    // Create and run verifier
    let verifier = Groth16Verifier::new(&proof_a, &proof_b, &proof_c, public_inputs, vk)
        .map_err(|_| ZkVerificationError::CreateGroth16VerifierFailed)?;
    
    verifier.verify().map_err(|_| ZkVerificationError::ProofVerificationFailed.into())
}

/// Verify a proof with variable-sized public inputs
pub fn verify_dynamic(
    public_inputs: &[[u8; 32]],
    proof: &CompressedProof,
    vk: &Groth16VerifyingKey,
) -> Result<()> {
    // Validate input count matches verifying key
    let expected_inputs = vk.gamma_abc_g1.len().saturating_sub(1);
    if public_inputs.len() != expected_inputs {
        msg!(
            "Public inputs length mismatch: expected {}, got {}",
            expected_inputs,
            public_inputs.len()
        );
        return Err(ZkVerificationError::InvalidPublicInputsLength.into());
    }
    
    // Decompress proof points
    let proof_a = decompress_g1(&proof.a)?;
    let proof_b = decompress_g2(&proof.b)?;
    let proof_c = decompress_g1(&proof.c)?;
    
    // Create and run verifier
    let verifier = Groth16Verifier::new(&proof_a, &proof_b, &proof_c, public_inputs, vk)
        .map_err(|_| ZkVerificationError::CreateGroth16VerifierFailed)?;
    
    verifier.verify().map_err(|_| ZkVerificationError::ProofVerificationFailed.into())
}
