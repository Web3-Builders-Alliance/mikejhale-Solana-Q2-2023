import * as anchor from '@project-serum/anchor';
import { BN, Program } from '@project-serum/anchor';
import { WbaVault } from '../target/types/wba_vault';
import { expect } from 'chai';
import {
  Connection,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from '@solana/spl-token';
import { parseMintAccount } from '@metaplex-foundation/js';

describe('wbavault', async () => {
  anchor.AnchorProvider.env().opts.commitment = 'confirmed';
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace.WbaVault as Program<WbaVault>;

  // Generate new keypair
  const keypair = anchor.web3.Keypair.generate();

  // Create a new keypair
  const vaultState = anchor.web3.Keypair.generate();

  // Create the PDA for our vault auth
  const vaultAuthSeeds = [Buffer.from('auth'), vaultState.publicKey.toBuffer()];

  const [vaultAuthKey, vaultAuthBump] = PublicKey.findProgramAddressSync(
    vaultAuthSeeds,
    program.programId
  );

  // Create the PDA for our vault
  const vaultSeeds = [Buffer.from('vault'), vaultAuthKey.toBuffer()];
  const [vaultKey, vaultBump] = PublicKey.findProgramAddressSync(
    vaultSeeds,
    program.programId
  );

  let mint = null;

  it('Airdrop token', async () => {
    const txhash = await provider.connection.requestAirdrop(
      keypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: txhash,
    });
  });

  it('Create Mint', async () => {
    mint = await createMint(
      provider.connection,
      keypair,
      keypair.publicKey,
      keypair.publicKey,
      6
    );

    let ownerAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      keypair,
      mint,
      keypair.publicKey
    );

    const txSig = await mintTo(
      provider.connection,
      keypair,
      mint,
      ownerAta.address,
      keypair,
      1500000000
    );

    const tokenAccount =
      await provider.connection.getParsedTokenAccountsByOwner(
        keypair.publicKey,
        {
          mint: mint,
        }
      );

    const tokenAmount = await provider.connection.getTokenAccountBalance(
      tokenAccount.value[0].pubkey
    );

    expect('1500000000').to.equal(tokenAmount.value.amount);
  });

  it('Is initialized', async () => {
    const txhash = await program.methods
      .initialize()
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vaultAuthKey,
        vault: vaultKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair, vaultState])
      .rpc();
  });

  it('Deposit', async () => {
    const vaultBeforeBalance = await provider.connection.getBalance(vaultKey);
    const txhash = await program.methods
      .deposit(new BN(0.1 * LAMPORTS_PER_SOL))
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vaultAuthKey,
        vault: vaultKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();

    const vaultAfterBalance = await provider.connection.getBalance(vaultKey);

    expect(vaultAfterBalance).to.equal(
      vaultBeforeBalance + 0.1 * LAMPORTS_PER_SOL
    );
  });

  it('Withdraw', async () => {
    const vaultBeforeBalance = await provider.connection.getBalance(vaultKey);
    const txhash = await program.methods
      .withdraw(new BN(0.1 * LAMPORTS_PER_SOL))
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vaultAuthKey,
        vault: vaultKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();

    const vaultAfterBalance = await provider.connection.getBalance(vaultKey);

    expect(vaultAfterBalance).to.equal(
      vaultBeforeBalance - 0.1 * LAMPORTS_PER_SOL
    );
  });

  it('Deposit SPL', async () => {
    let ownerAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      keypair,
      mint,
      keypair.publicKey
    );

    let vaultAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      keypair,
      mint,
      vaultAuthKey,
      true
    );

    const txhash = await program.methods
      .depositSpl(new BN(0.1 * LAMPORTS_PER_SOL))
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        ownerAta: ownerAta.address,
        vaultAta: vaultAta.address,
        tokenMint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();
  });

  /*
  it('Withdraw SPL', async () => {
    let ownerAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      keypair,
      mint,
      keypair.publicKey
    );

    let vaultAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      keypair,
      mint,
      vaultAuthKey,
      true
    );

    const vaultBeforeBalance = await provider.connection.getBalance(vaultKey);
    const txhash = await program.methods
      .withdraw_spl(new BN(0.1 * LAMPORTS_PER_SOL))
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vaultAuthKey,
        ownerAta: ownerAta.address,
        vaultAta: vaultAta.address,
        tokenMint: mint,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();

    const vaultAfterBalance = await provider.connection.getBalance(vaultKey);

    expect(vaultAfterBalance).to.equal(
      vaultBeforeBalance - 0.1 * LAMPORTS_PER_SOL
    );
  });
  */
});
