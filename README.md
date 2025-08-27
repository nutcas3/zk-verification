# zk-verification

A Zero-Knowledge (ZK) proof-based project exploring privacy-preserving compliance primitives on Solana/Anchor.

## Overview: ZK for Compliance

Zero-Knowledge (ZK) proofs are a powerful tool for a variety of compliance challenges, allowing organizations to satisfy regulatory requirements without compromising user privacy or revealing sensitive business information. Here are several ideas for how ZK technology can be applied to compliance:

### 1. ZK-KYC (Know Your Customer) and AML (Anti-Money Laundering)
This is one of the most prominent applications. Traditional KYC requires a user to submit sensitive documents (passports, driver's licenses) to a service provider, which then stores this data, creating a liability and a target for hackers.

- **The ZK Idea:** A user verifies their identity with a trusted, centralized issuer (e.g., a government or a specialized identity provider). This issuer provides the user with a privacy-preserving digital credential. When a service (like a DeFi protocol or a traditional bank) needs to verify the user's identity, the user generates a ZK proof from their credential. This proof can state, for example, "I am a citizen of Country X," or "I am on the approved list of users and am not on any sanctions list," without revealing the user's name, address, or other personal details.
- **Compliance Benefit:** The service provider can prove to regulators that it has performed the necessary KYC/AML checks without ever holding the user's sensitive data, thereby meeting compliance obligations while drastically reducing data risk.

### 2. Private Audits and Financial Reporting
Auditors and regulators often need to review a company's financial data to ensure compliance. This process can be slow, intrusive, and often exposes sensitive financial information.

- **The ZK Idea:** A company can use ZK proofs to demonstrate the validity of its financial statements. For example, a company could prove that its reported revenue equals the sum of all individual transactions, or that its balance sheet is correctly calculated, without revealing any of the underlying transaction data. Similarly, a public company could prove that it has at least $1 million in reserves without revealing the exact amount or the location of the funds.
- **Compliance Benefit:** Regulators and auditors can verify a company's compliance with financial reporting standards and regulations without gaining access to proprietary and competitive information. This can make audits more efficient and less burdensome.

### 3. GDPR and Other Data Privacy Regulations
Regulations like the GDPR give individuals control over their data and place strict requirements on how companies handle and store it. ZK proofs are an elegant solution to some of the most challenging aspects of these rules.

- **The ZK Idea:** A company can use ZK proofs to demonstrate that it is only processing personal data in a way that is compliant with a user's explicit consent. For instance, a proof could show that a marketing campaign only targets users who have agreed to be part of such a campaign, without revealing the identities of those users. Furthermore, a user could request a "zero-knowledge deletion" where they receive a proof that their data has been erased from a company's database, without the company having to reveal the entire database to prove the deletion.
- **Compliance Benefit:** Companies can satisfy the core principles of data minimization and purpose limitation by using ZK proofs to verify compliance with consent rules and data handling policies without ever having to expose the underlying personal data.

### 4. Supply Chain and ESG Reporting
Regulators are increasingly requiring companies to be transparent about their supply chains and environmental, social, and governance (ESG) practices. Proving the origin of goods or the ethical sourcing of materials can be complex and difficult.

- **The ZK Idea:** A supply chain participant can generate a ZK proof that a product was sourced from a specific, ethically certified region, without revealing the name of the supplier or the exact cost of the materials. This could also be used to prove that a product contains a certain percentage of recycled materials without revealing the full formula or manufacturing process.
- **Compliance Benefit:** This provides a verifiable way for companies to prove compliance with ESG regulations and ethical sourcing standards, increasing consumer trust while protecting sensitive business secrets.

### 5. Private Sanctions Screening
Financial institutions are required to screen all transactions against sanctions lists. This usually involves exposing transaction details to a third-party screening service.

- **The ZK Idea:** A financial institution can generate a ZK proof that a specific transaction's parties (sender and receiver) are not on a sanctions list, without revealing their identities or the transaction amount to the screening service. The service simply verifies the proof, attesting to its validity.
- **Compliance Benefit:** This ensures that sanctions are being enforced while preserving the privacy of individuals and entities not on the list. It also protects the financial institution from having to disclose sensitive client data to third-party providers.

By shifting the paradigm from "trust me, I'm compliant" to "verify my compliance without seeing my data," ZK proofs offer a way to enforce rules and regulations in a more secure, private, and efficient manner.

---

## zkVerify: A Fleshed-Out ZK-KYC Service

The provided text offers an excellent foundation for a ZK-KYC idea. To "flesh it out," we can expand on the initial concept by creating a more detailed and actionable plan. This involves moving from the theoretical "what" and "why" to the practical "how" and "for whom."

### 1. The Problem: A Deeper Dive
The core problem is the conflict between regulatory compliance (KYC) and individual privacy. We can elaborate on this by highlighting specific pain points for all parties involved:

- **For Users:**
  - **Privacy Invasion:** Forced to hand over sensitive personal data (passport scans, SSNs, addresses) to multiple, often unknown, service providers. This data is a "honeypot" for hackers.
  - **Data Breach Risk:** Constant worry about their data being leaked, sold, or misused.
  - **Friction and Repetition:** The manual, time-consuming process of re-submitting and re-verifying documents for every new service.
  - **Lack of Control:** Once the data is submitted, the user loses all control over it.
- **For Businesses:**
  - **Regulatory Burden:** The high cost and complexity of setting up and maintaining a compliant KYC system.
  - **Data Storage Liability:** The legal and financial risk of storing vast amounts of sensitive user data. A single data breach can result in massive fines and reputational damage.
  - **High Onboarding Friction:** The slow and cumbersome KYC process leads to high user drop-off rates.
  - **Inefficiency:** Manual review processes are expensive and prone to human error.
- **For Regulators:**
  - **Enforcement Challenges:** Difficulty in auditing and verifying that businesses are handling user data securely.
  - **Ineffective Compliance:** Even with strict rules, data breaches still happen, meaning the current system isn't foolproof.

### 2. The Solution: A Fleshed-Out ZK-KYC Service
Let's imagine a specific ZK-KYC product, which we'll call "zkVerify."

- **Product Vision:** A user-centric, privacy-preserving identity verification service that enables businesses to comply with regulations while eliminating the need to store sensitive personal data.

- **Key Features of zkVerify:**
  - **User-Controlled Identity Wallet:** A secure mobile application where users store their Verifiable Credentials (VCs). This wallet is their "digital passport," and all data remains on their device.
  - **Credential Issuance Network:** A network of accredited "Issuers" (e.g., government partners, major banks, or specialized identity verification companies) who are authorized to issue VCs after an initial, traditional KYC process.
  - **On-Demand Zero-Knowledge Proofs:** When a service provider (a "Verifier") needs to confirm a user's identity, the user receives a "verification request." The zkVerify wallet then generates a ZK proof that satisfies the request (e.g., "This user is over 21," "This user's country of residence is USA," "This user is not on a sanctions list") without revealing the underlying data.
  - **Blockchain Integration:** The ZK proof is a small, cryptographically secure file that can be posted on a public or private blockchain (e.g., as a non-transferable Soulbound NFT). This token acts as an immutable, verifiable proof of a user's compliance status.
  - **API for Verifiers:** A simple and robust API for businesses to integrate zkVerify. Instead of receiving and storing user documents, they receive a simple "Proof of Compliance" and a verified token address.

### 3. Business Model and Monetization
How would a company like zkVerify make money?

- **Tiered API Subscriptions for Businesses:**
  - **Free Tier:** For small businesses, allowing a limited number of proofs per month.
  - **Pro Tier:** A monthly subscription fee for a higher volume of verifications.
  - **Enterprise Tier:** Custom pricing for large corporations with high-volume needs, offering dedicated support and on-premise integration.
- **Proof Generation Fees (per-proof or batch):** A small fee charged to the Verifier for each successful ZK proof verification. This is a common model for blockchain-based services.
- **SaaS for Credential Issuers:** Charging a fee to the trusted "Issuers" who want to be part of the zkVerify network. This could be a recurring subscription or a fee per credential issued.

### 4. Target Market and Use Cases
The target market is any industry that requires KYC, with a special focus on those with a high volume of digital interactions or a strong need for data privacy.

- **DeFi and Web3:** The most obvious fit. Protocols can verify users for compliance, for example, to create "whitelists" for token launches or to meet AML requirements, without compromising the decentralized ethos.
- **Fintech & Traditional Banks:** Integrating zkVerify to streamline their digital onboarding process, drastically reducing friction and user drop-off.
- **Gaming & Metaverse:** Verifying a user's age for content restrictions or to combat bot accounts without revealing their identity.
- **Social Media:** Proving "personhood" to fight bots and spam, ensuring that a user is a real human without requiring them to link to their personal identity.
- **Online Voting Platforms:** Enabling a user to prove they are an eligible voter without revealing their identity or their vote.
- **HealthTech:** Allowing a user to prove they meet a certain medical condition (e.g., vaccination status) to a service provider without revealing the full medical record.

### 5. Competitive Advantage
What makes zkVerify stand out?

- **Privacy-by-Design:** Unlike traditional KYC providers, our core value proposition is the elimination of user data storage, not just secure storage.
- **Reusable Credentials:** Users can generate new proofs for different services with their single, verified credential, creating a network effect.
- **Blockchain-Verified Immutability:** The use of non-transferable tokens on-chain provides an unforgeable, publicly verifiable record of compliance without revealing any private information.
- **Regulatory-First Approach:** Proactively working with regulators to define standards for ZK-KYC, positioning the service as a leader in compliant, privacy-preserving technology.
- **Simplified Integration:** A user-friendly API and SDKs for developers, making it easy for businesses to adopt the technology.

### 6. Go-to-Market Strategy

- **Pilot Programs:** Partner with a few key DeFi projects and fintech startups to test the service and build a case study.
- **Developer Relations:** Create comprehensive documentation, tutorials, and a community for developers to build on the platform.
- **Regulatory Advocacy:** Engage with global regulators and standards bodies to promote ZK-KYC as the new standard for identity verification.
- **Content Marketing:** Publish articles and case studies on the benefits of ZK-KYC, targeting both businesses and consumers.

---

## Project Setup

- **Prerequisites**
  - Rust toolchain and Cargo
  - Solana CLI (v1.18+)
  - Anchor CLI (v0.29+)
  - Node.js 18+ and pnpm/npm

- **Install dependencies**

  ```bash
  npm install
  ```

- **Build on-chain program**

  ```bash
  anchor build
  ```

- **Run tests**

  ```bash
  anchor test
  ```

- **Local validator (optional)**

  ```bash
  solana-test-validator
  ```

## Project Structure

- `programs/zk-verification/` — Anchor program skeleton for ZK verification flows
- `tests/zk-verification.ts` — TypeScript tests using Anchor Mocha provider
- `migrations/deploy.ts` — Anchor deploy script
- `Anchor.toml` — Workspace configuration
- `Cargo.toml` (root/program) — Rust crates configuration

## Architecture Overview

- **On-chain program (`programs/zk-verification/`)**
  - Goal: verify succinct proofs or commitments emitted off-chain and record minimal compliance state on-chain.
  - Near-term: stub instruction(s) that accept a proof artifact (bytes) and a verifying key/pubkey reference, perform checks (placeholder), and emit events.
- **Off-chain proving**
  - Proof generation/verification likely handled by an off-chain service or client SDK initially.
  - Future: integrate with Solana-compatible ZK libraries or precompiles when feasible.
- **Events and state**
  - Prefer event logs for attestations over persistent PII or sensitive state. If state is needed, store only hashed commitments.

## Integration Guide

### On-Chain Program Interface

The ZK verification program provides the following instructions:

```rust
// Initialize the program
initialize()

// Register a new verifying key for a specific circuit
register_verifying_key(
    circuit_id: String,
    key: Groth16VerifyingKey,
)

// Verify a ZK proof
verify_proof(
    verification_context: VerificationContext,
)
```

### Data Structures

```rust
// Compressed representation of a proof for efficient on-chain storage
struct CompressedProof {
    a: [u8; 32],  // Compressed G1 point for proof.a
    b: [u8; 64],  // Compressed G2 point for proof.b
    c: [u8; 32],  // Compressed G1 point for proof.c
}

// Verifying key for Groth16 proofs
struct Groth16VerifyingKey {
    alpha_g1: [u8; 32],     // Alpha in G1
    beta_g2: [u8; 64],      // Beta in G2
    gamma_g2: [u8; 64],     // Gamma in G2
    delta_g2: [u8; 64],     // Delta in G2
    gamma_abc_g1: Vec<[u8; 32]>,  // Gamma ABC G1 elements
}

// Verification context for a specific proof type
struct VerificationContext {
    verification_type: String,  // Type of verification (e.g., "kyc", "sanctions", "age")
    public_inputs: Vec<[u8; 32]>,  // Public inputs for the verification
    proof: CompressedProof,  // The compressed proof
}
```

### Client SDK Usage

Here's how to use the ZK verification system from a TypeScript client:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { ZkVerification } from "../target/types/zk_verification";

// Connect to the program
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.ZkVerification as anchor.Program<ZkVerification>;

// Register a verifying key
async function registerVerifyingKey() {
  const verifyingKeyAccount = Keypair.generate();
  
  // Create mock verifying key (in production, this would be a real verifying key)
  const verifyingKey = {
    alphaG1: Buffer.from(Array(32).fill(1)),
    betaG2: Buffer.from(Array(64).fill(1)),
    gammaG2: Buffer.from(Array(64).fill(1)),
    deltaG2: Buffer.from(Array(64).fill(1)),
    gammaAbcG1: [
      Buffer.from(Array(32).fill(1)),
      Buffer.from(Array(32).fill(1)),
      Buffer.from(Array(32).fill(1)),
    ],
  };
  
  // Register the verifying key
  const tx = await program.methods
    .registerVerifyingKey(
      "kyc_verification",
      verifyingKey
    )
    .accounts({
      authority: provider.wallet.publicKey,
      verifyingKey: verifyingKeyAccount.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([verifyingKeyAccount])
    .rpc();
  
  console.log("Verifying key registered:", tx);
  return verifyingKeyAccount.publicKey;
}

// Verify a ZK proof
async function verifyProof(verifyingKeyPubkey, subject) {
  const verificationResultAccount = Keypair.generate();
  
  // In a real implementation, this would be generated from a ZK circuit
  const proof = {
    a: Buffer.from(Array(32).fill(2)),
    b: Buffer.from(Array(64).fill(2)),
    c: Buffer.from(Array(32).fill(2)),
  };
  
  // Public inputs (e.g., for KYC: is_adult_flag and country_code_hash)
  const publicInputs = [
    Buffer.from(Array(32).fill(0).map((_, i) => i === 0 ? 1 : 0)),  // is_adult = true
    Buffer.from(Array(32).fill(0).map((_, i) => i < 2 ? "US".charCodeAt(i) : 0)),  // country = "US"
  ];
  
  // Verify the proof
  const tx = await program.methods
    .verifyProof({
      verificationType: "age_verification",
      publicInputs: publicInputs,
      proof: proof,
    })
    .accounts({
      authority: provider.wallet.publicKey,
      subject: subject,
      verifyingKey: verifyingKeyPubkey,
      verificationResult: verificationResultAccount.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([verificationResultAccount])
    .rpc();
  
  console.log("Proof verified:", tx);
  
  // Fetch the verification result
  const result = await program.account.verificationResult.fetch(
    verificationResultAccount.publicKey
  );
  
  return result;
}
```

### Integration Patterns

#### 1. KYC Verification Flow

1. **Setup Phase**:
   - Authority registers a verifying key for KYC verification
   - Users complete KYC with a trusted issuer who provides them with a credential

2. **Verification Phase**:
   - User generates a ZK proof from their credential (off-chain)
   - User submits the proof to your dApp
   - Your dApp calls the `verify_proof` instruction
   - The program verifies the proof and stores the result

3. **Access Control Phase**:
   - Your dApp checks the verification result before granting access
   - The verification result can be reused until it expires

#### 2. Age Verification Flow

1. **Setup Phase**:
   - Authority registers a verifying key for age verification

2. **Verification Phase**:
   - User generates a ZK proof that they are over a certain age (off-chain)
   - The proof reveals nothing about the user's actual age or identity
   - Your dApp verifies the proof on-chain

3. **Access Control Phase**:
   - Grant access to age-restricted content or features based on the verification result

#### 3. Sanctions Screening Flow

1. **Setup Phase**:
   - Authority registers a verifying key for sanctions screening

2. **Verification Phase**:
   - User generates a ZK proof that they are not on a sanctions list (off-chain)
   - The proof reveals nothing about the user's identity
   - Your dApp verifies the proof on-chain

3. **Transaction Phase**:
   - Allow transactions only with verified counterparties

## Roadmap

- **Phase 0: Repo hygiene**
  - README, CI skeleton, fmt/lint
- **Phase 1: Program scaffold**
  - Add `verify_proof` stub instruction and events
  - Basic tests invoking instruction
- **Phase 2: Off-chain prover adapter**
  - Define proof format contract (e.g., Groth16/Plonk placeholder)
  - Mock verifier off-chain; on-chain checks minimal structure
- **Phase 3: Commitment & revocation**
  - Store hashed commitments; add revocation/update flows
- **Phase 4: Integrations**
  - Example flows for zk-KYC, sanctions screening
- **Phase 5: Security & audits**
  - Threat modeling, fuzzing, and external review

## Compliance & Threat Model Notes

- **Data minimization:** Never store raw PII on-chain. Prefer commitments and short-lived attestations.
- **Selective disclosure:** Scope proofs to specific claims (age, residency, sanctions) rather than full identity.
- **Revocation:** Design for issuer/user-driven revocation of credentials/attestations via hashed registries.
- **Regulatory alignment:** Map claims to jurisdictional requirements (e.g., KYC levels, AML checks) and log proof metadata for auditability without sensitive data.
- **Abuse resistance:** Consider Sybil resistance and rate-limiting without deanonymization (proof-of-personhood commitments, nullifiers).
- **Key management:** Clear guidance for issuers/verifiers on key rotation and verifying key provenance.
