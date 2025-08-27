import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ZkVerification } from "../target/types/zk_verification";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { BN } from "bn.js";

// Mock ZK proof generation - in a real implementation, this would use a ZK library
interface CompressedProof {
  a: number[];
  b: number[];
  c: number[];
}

interface Groth16VerifyingKey {
  alphaG1: number[];
  betaG2: number[];
  gammaG2: number[];
  deltaG2: number[];
  gammaAbcG1: number[][];
}

// Mock proof generation for KYC verification
function generateKycProof(userAge: number, countryCode: string): {
  proof: CompressedProof;
  publicInputs: number[][];
} {
  // In a real implementation, this would use a ZK circuit to generate a proof
  // For this example, we're just creating mock data
  
  // Mock proof
  const proof: CompressedProof = {
    a: Array(32).fill(2),
    b: Array(64).fill(2),
    c: Array(32).fill(2),
  };
  
  // Public inputs: [is_adult_flag, country_code_hash]
  // In a real implementation, these would be properly formatted field elements
  const isAdult = userAge >= 21 ? 1 : 0;
  
  // Simple encoding of country code
  const countryBytes = Array(32).fill(0);
  for (let i = 0; i < countryCode.length && i < 2; i++) {
    countryBytes[i] = countryCode.charCodeAt(i);
  }
  
  const publicInputs = [
    Array(32).fill(0).map((_, i) => i === 0 ? isAdult : 0),
    countryBytes
  ];
  
  return { proof, publicInputs };
}

// Mock verifying key generation
function getKycVerifyingKey(): Groth16VerifyingKey {
  // In a real implementation, this would be a properly generated verifying key
  return {
    alphaG1: Array(32).fill(1),
    betaG2: Array(64).fill(1),
    gammaG2: Array(64).fill(1),
    deltaG2: Array(64).fill(1),
    gammaAbcG1: [
      Array(32).fill(1), // Constant term
      Array(32).fill(1), // Coefficient for "is_adult"
      Array(32).fill(1), // Coefficient for "country_code_hash"
    ],
  };
}

describe("zk-verification", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const program = anchor.workspace.ZkVerification as Program<ZkVerification>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  
  // Test accounts
  const authority = provider.wallet;
  const subject = Keypair.generate();
  let verifyingKeyAccount: Keypair;
  let verificationResultAccount: Keypair;
  
  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Initialization transaction signature", tx);
  });
  
  it("Can register a verifying key", async () => {
    verifyingKeyAccount = Keypair.generate();
    
    // Get mock verifying key
    const verifyingKey = getKycVerifyingKey();
    
    // Register the verifying key
    const tx = await program.methods
      .registerVerifyingKey(
        "kyc_verification",
        {
          alphaG1: Buffer.from(verifyingKey.alphaG1),
          betaG2: Buffer.from(verifyingKey.betaG2),
          gammaG2: Buffer.from(verifyingKey.gammaG2),
          deltaG2: Buffer.from(verifyingKey.deltaG2),
          gammaAbcG1: verifyingKey.gammaAbcG1.map(x => Buffer.from(x)),
        }
      )
      .accounts({
        authority: authority.publicKey,
        verifyingKey: verifyingKeyAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([verifyingKeyAccount])
      .rpc();
    
    console.log("Register verifying key transaction signature", tx);
  });
  
  it("Can verify a ZK proof", async () => {
    verificationResultAccount = Keypair.generate();
    
    // Generate a proof that the user is over 21 and from the US
    const { proof, publicInputs } = generateKycProof(25, "US");
    
    // Verify the proof
    const tx = await program.methods
      .verifyProof({
        verificationType: "age_verification",
        publicInputs: publicInputs.map(x => Buffer.from(x)),
        proof: {
          a: Buffer.from(proof.a),
          b: Buffer.from(proof.b),
          c: Buffer.from(proof.c),
        },
      })
      .accounts({
        authority: authority.publicKey,
        subject: subject.publicKey,
        verifyingKey: verifyingKeyAccount.publicKey,
        verificationResult: verificationResultAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([verificationResultAccount])
      .rpc();
    
    console.log("Verify proof transaction signature", tx);
    
    // Fetch the verification result
    const result = await program.account.verificationResult.fetch(
      verificationResultAccount.publicKey
    );
    
    console.log("Verification result:", {
      subject: result.subject.toString(),
      verifier: result.verifier.toString(),
      verificationType: result.verificationType,
      timestamp: result.timestamp.toString(),
      isValid: result.isValid,
      commitment: result.commitment ? Buffer.from(result.commitment).toString('hex') : null,
    });
  });
});
