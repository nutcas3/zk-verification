use anchor_lang::prelude::*;
use crate::errors::ZkVerificationError;

pub const MAX_PUBLIC_INPUTS: usize = 32;

pub struct PublicInputsHandler {
    inputs: Vec<[u8; 32]>,
}

impl PublicInputsHandler {
    pub fn new(inputs: Vec<[u8; 32]>) -> Result<Self> {
        if inputs.is_empty() {
            return Err(ZkVerificationError::InvalidPublicInputsLength.into());
        }
        
        if inputs.len() > MAX_PUBLIC_INPUTS {
            return Err(ZkVerificationError::InvalidPublicInputsLength.into());
        }
        
        Ok(Self { inputs })
    }
    
    pub fn len(&self) -> usize {
        self.inputs.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }
    
    pub fn as_slice(&self) -> &[[u8; 32]] {
        &self.inputs
    }
    
    pub fn get(&self, index: usize) -> Option<&[u8; 32]> {
        self.inputs.get(index)
    }
    
    pub fn validate_count(&self, expected: usize) -> Result<()> {
        if self.inputs.len() != expected {
            msg!(
                "Invalid public inputs count: expected {}, got {}",
                expected,
                self.inputs.len()
            );
            return Err(ZkVerificationError::InvalidPublicInputsLength.into());
        }
        Ok(())
    }
    
    pub fn validate_against_vk(&self, vk_gamma_abc_len: usize) -> Result<()> {
        let expected_inputs = vk_gamma_abc_len.saturating_sub(1);
        self.validate_count(expected_inputs)
    }
    
    pub fn to_fixed_array<const N: usize>(&self) -> Result<[[u8; 32]; N]> {
        if self.inputs.len() != N {
            return Err(ZkVerificationError::InvalidPublicInputsLength.into());
        }
        
        let mut array = [[0u8; 32]; N];
        for (i, input) in self.inputs.iter().enumerate() {
            array[i] = *input;
        }
        Ok(array)
    }
    
    pub fn extract_commitment(&self) -> Option<[u8; 32]> {
        self.inputs.first().copied()
    }
    
    pub fn all_zero(&self) -> bool {
        self.inputs.iter().all(|input| input.iter().all(|&b| b == 0))
    }
}

pub trait ToPublicInputs {
    fn to_public_inputs(&self) -> Vec<[u8; 32]>;
}

pub mod patterns {
    use super::*;
    
    pub struct AgeVerification {
        pub age_commitment: [u8; 32],
        pub minimum_age: u8,
        pub current_timestamp: i64,
    }
    
    impl ToPublicInputs for AgeVerification {
        fn to_public_inputs(&self) -> Vec<[u8; 32]> {
            let mut inputs = Vec::new();
            inputs.push(self.age_commitment);
            
            let mut min_age_bytes = [0u8; 32];
            min_age_bytes[0] = self.minimum_age;
            inputs.push(min_age_bytes);
            
            let mut timestamp_bytes = [0u8; 32];
            timestamp_bytes[..8].copy_from_slice(&self.current_timestamp.to_le_bytes());
            inputs.push(timestamp_bytes);
            
            inputs
        }
    }
    
    pub struct KycVerification {
        pub identity_commitment: [u8; 32],
        pub country_code_hash: [u8; 32],
        pub sanctions_list_hash: [u8; 32],
    }
    
    impl ToPublicInputs for KycVerification {
        fn to_public_inputs(&self) -> Vec<[u8; 32]> {
            vec![
                self.identity_commitment,
                self.country_code_hash,
                self.sanctions_list_hash,
            ]
        }
    }
    
    pub struct AccreditedInvestorVerification {
        pub investor_commitment: [u8; 32],
        pub net_worth_threshold: u64,
        pub income_threshold: u64,
    }
    
    impl ToPublicInputs for AccreditedInvestorVerification {
        fn to_public_inputs(&self) -> Vec<[u8; 32]> {
            let mut inputs = Vec::new();
            inputs.push(self.investor_commitment);
            
            let mut net_worth_bytes = [0u8; 32];
            net_worth_bytes[..8].copy_from_slice(&self.net_worth_threshold.to_le_bytes());
            inputs.push(net_worth_bytes);
            
            let mut income_bytes = [0u8; 32];
            income_bytes[..8].copy_from_slice(&self.income_threshold.to_le_bytes());
            inputs.push(income_bytes);
            
            inputs
        }
    }
    
    pub struct CreditScoreVerification {
        pub credit_commitment: [u8; 32],
        pub minimum_score: u16,
        pub verification_timestamp: i64,
    }
    
    impl ToPublicInputs for CreditScoreVerification {
        fn to_public_inputs(&self) -> Vec<[u8; 32]> {
            let mut inputs = Vec::new();
            inputs.push(self.credit_commitment);
            
            let mut score_bytes = [0u8; 32];
            score_bytes[..2].copy_from_slice(&self.minimum_score.to_le_bytes());
            inputs.push(score_bytes);
            
            let mut timestamp_bytes = [0u8; 32];
            timestamp_bytes[..8].copy_from_slice(&self.verification_timestamp.to_le_bytes());
            inputs.push(timestamp_bytes);
            
            inputs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use patterns::*;
    
    #[test]
    fn test_public_inputs_handler() {
        let inputs = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        let handler = PublicInputsHandler::new(inputs.clone()).unwrap();
        
        assert_eq!(handler.len(), 3);
        assert!(!handler.is_empty());
        assert_eq!(handler.get(0), Some(&[1u8; 32]));
    }
    
    #[test]
    fn test_validate_count() {
        let inputs = vec![[1u8; 32], [2u8; 32]];
        let handler = PublicInputsHandler::new(inputs).unwrap();
        
        assert!(handler.validate_count(2).is_ok());
        assert!(handler.validate_count(3).is_err());
    }
    
    #[test]
    fn test_age_verification_pattern() {
        let age_proof = AgeVerification {
            age_commitment: [1u8; 32],
            minimum_age: 21,
            current_timestamp: 1234567890,
        };
        
        let inputs = age_proof.to_public_inputs();
        assert_eq!(inputs.len(), 3);
        assert_eq!(inputs[0], [1u8; 32]);
        assert_eq!(inputs[1][0], 21);
    }
    
    #[test]
    fn test_kyc_verification_pattern() {
        let kyc_proof = KycVerification {
            identity_commitment: [1u8; 32],
            country_code_hash: [2u8; 32],
            sanctions_list_hash: [3u8; 32],
        };
        
        let inputs = kyc_proof.to_public_inputs();
        assert_eq!(inputs.len(), 3);
    }
}
