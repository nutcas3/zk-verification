import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  Keypair,
  SystemProgram,
} from '@solana/web3.js';
import { Program, AnchorProvider, BN, web3 } from '@coral-xyz/anchor';
import { IDL, ZkVerification } from './idl';

export interface VerificationContext {
  verificationType: string;
  publicInputs: Buffer[];
  proof: CompressedProof;
}

export interface CompressedProof {
  a: Buffer;
  b: Buffer;
  c: Buffer;
}

export interface CredentialMetadata {
  level: CredentialLevel;
  issuerAuthority: PublicKey;
  validFrom: BN;
  validUntil: BN;
  revocationRegistry: PublicKey;
  jurisdiction: Buffer;
}

export enum CredentialLevel {
  Basic = 'Basic',
  Standard = 'Standard',
  Enhanced = 'Enhanced',
  Institutional = 'Institutional',
}

export enum ProofSystem {
  Groth16 = 'Groth16',
  Plonk = 'Plonk',
  Stark = 'Stark',
  Halo2 = 'Halo2',
}

export class ZkVerificationClient {
  private program: Program<ZkVerification>;
  private connection: Connection;
  private provider: AnchorProvider;

  constructor(
    connection: Connection,
    wallet: any,
    programId: PublicKey
  ) {
    this.connection = connection;
    this.provider = new AnchorProvider(connection, wallet, {
      commitment: 'confirmed',
    });
    this.program = new Program(IDL, programId, this.provider);
  }

  /**
   * Initialize the nullifier registry (one-time setup)
   */
  async initializeNullifierRegistry(
    authority: Keypair
  ): Promise<string> {
    const [nullifierRegistry] = PublicKey.findProgramAddressSync(
      [Buffer.from('nullifier_registry')],
      this.program.programId
    );

    const tx = await this.program.methods
      .initializeNullifierRegistry()
      .accounts({
        authority: authority.publicKey,
        nullifierRegistry,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    return tx;
  }

  /**
   * Verify a ZK proof with nullifier protection
   */
  async verifyProof(
    authority: Keypair,
    subject: PublicKey,
    verifyingKey: PublicKey,
    verificationContext: VerificationContext,
    nullifierHash: Buffer
  ): Promise<string> {
    const [nullifier] = PublicKey.findProgramAddressSync(
      [Buffer.from('nullifier'), nullifierHash],
      this.program.programId
    );

    const [nullifierRegistry] = PublicKey.findProgramAddressSync(
      [Buffer.from('nullifier_registry')],
      this.program.programId
    );

    const verificationResult = Keypair.generate();

    const tx = await this.program.methods
      .verifyProof(verificationContext, Array.from(nullifierHash))
      .accounts({
        authority: authority.publicKey,
        subject,
        verifyingKey,
        verificationResult: verificationResult.publicKey,
        nullifier,
        nullifierRegistry,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority, verificationResult])
      .rpc();

    return tx;
  }

  /**
   * Register a universal verifier for any proof system
   */
  async registerUniversalVerifier(
    authority: Keypair,
    proofSystem: ProofSystem,
    circuitId: string,
    verifyingKey: Buffer
  ): Promise<PublicKey> {
    const universalVerifier = Keypair.generate();

    await this.program.methods
      .registerUniversalVerifier(
        { [proofSystem.toLowerCase()]: {} },
        circuitId,
        Array.from(verifyingKey)
      )
      .accounts({
        authority: authority.publicKey,
        universalVerifier: universalVerifier.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority, universalVerifier])
      .rpc();

    return universalVerifier.publicKey;
  }

  /**
   * Register a trusted issuer
   */
  async registerIssuer(
    authority: Keypair,
    issuerAuthority: Keypair,
    name: string,
    jurisdiction: Buffer,
    credentialTypes: string[],
    stakeAmount: BN
  ): Promise<{ issuer: PublicKey; stake: PublicKey }> {
    const [issuerRegistry] = PublicKey.findProgramAddressSync(
      [Buffer.from('issuer_registry')],
      this.program.programId
    );

    const [issuer] = PublicKey.findProgramAddressSync(
      [Buffer.from('issuer'), issuerAuthority.publicKey.toBuffer()],
      this.program.programId
    );

    const [issuerStake] = PublicKey.findProgramAddressSync(
      [Buffer.from('issuer_stake'), issuerAuthority.publicKey.toBuffer()],
      this.program.programId
    );

    await this.program.methods
      .registerIssuer(
        name,
        Array.from(jurisdiction),
        credentialTypes,
        stakeAmount
      )
      .accounts({
        authority: authority.publicKey,
        issuerAuthority: issuerAuthority.publicKey,
        issuerRegistry,
        issuer,
        issuerStake,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority, issuerAuthority])
      .rpc();

    return { issuer, stake: issuerStake };
  }

  /**
   * Issue a credential to a user
   */
  async issueCredential(
    issuerAuthority: Keypair,
    owner: PublicKey,
    metadata: CredentialMetadata,
    commitment: Buffer
  ): Promise<PublicKey> {
    const [issuer] = PublicKey.findProgramAddressSync(
      [Buffer.from('issuer'), issuerAuthority.publicKey.toBuffer()],
      this.program.programId
    );

    const [credential] = PublicKey.findProgramAddressSync(
      [Buffer.from('credential'), owner.toBuffer(), issuer.toBuffer()],
      this.program.programId
    );

    await this.program.methods
      .issueCredential(metadata, Array.from(commitment))
      .accounts({
        issuerAuthority: issuerAuthority.publicKey,
        owner,
        issuer,
        credential,
        systemProgram: SystemProgram.programId,
      })
      .signers([issuerAuthority])
      .rpc();

    return credential;
  }

  /**
   * Revoke a credential
   */
  async revokeCredential(
    authority: Keypair,
    credential: PublicKey,
    reason: string
  ): Promise<string> {
    const credentialAccount = await this.program.account.credential.fetch(
      credential
    );

    const [revocationRegistry] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('revocation_registry'),
        credentialAccount.metadata.issuerAuthority.toBuffer(),
      ],
      this.program.programId
    );

    const [issuer] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('issuer'),
        credentialAccount.metadata.issuerAuthority.toBuffer(),
      ],
      this.program.programId
    );

    const tx = await this.program.methods
      .revokeCredential(reason)
      .accounts({
        authority: authority.publicKey,
        credential,
        revocationRegistry,
        issuer,
      })
      .signers([authority])
      .rpc();

    return tx;
  }

  /**
   * Perform selective disclosure
   */
  async selectiveDisclosure(
    subject: Keypair,
    verifier: PublicKey,
    request: any,
    nullifier: Buffer,
    expiresIn: BN
  ): Promise<PublicKey> {
    const [credential] = PublicKey.findProgramAddressSync(
      [Buffer.from('credential'), subject.publicKey.toBuffer()],
      this.program.programId
    );

    const disclosureProof = Keypair.generate();

    await this.program.methods
      .selectiveDisclosure(request, Array.from(nullifier), expiresIn)
      .accounts({
        subject: subject.publicKey,
        verifier,
        credential,
        disclosureProof: disclosureProof.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([subject, disclosureProof])
      .rpc();

    return disclosureProof.publicKey;
  }

  /**
   * Get verification result
   */
  async getVerificationResult(address: PublicKey) {
    return await this.program.account.verificationResult.fetch(address);
  }

  /**
   * Get credential
   */
  async getCredential(address: PublicKey) {
    return await this.program.account.credential.fetch(address);
  }

  /**
   * Get issuer information
   */
  async getIssuer(issuerAuthority: PublicKey) {
    const [issuer] = PublicKey.findProgramAddressSync(
      [Buffer.from('issuer'), issuerAuthority.toBuffer()],
      this.program.programId
    );
    return await this.program.account.trustedIssuer.fetch(issuer);
  }

  /**
   * Check if nullifier has been used
   */
  async isNullifierUsed(nullifierHash: Buffer): Promise<boolean> {
    const [nullifier] = PublicKey.findProgramAddressSync(
      [Buffer.from('nullifier'), nullifierHash],
      this.program.programId
    );

    try {
      await this.program.account.proofNullifier.fetch(nullifier);
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Get nullifier registry stats
   */
  async getNullifierRegistryStats() {
    const [nullifierRegistry] = PublicKey.findProgramAddressSync(
      [Buffer.from('nullifier_registry')],
      this.program.programId
    );
    return await this.program.account.nullifierRegistry.fetch(
      nullifierRegistry
    );
  }
}
