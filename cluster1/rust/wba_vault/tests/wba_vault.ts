import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { WbaVault } from '../target/types/wba_vault';
import {
  Connection,
  PublicKey,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
} from '@solana/web3.js';

describe('wba_vault', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.WbaVault as Program<WbaVault>;
  const keypair = anchor.web3.Keypair.generate();
  const vaultState = anchor.web3.Keypair.generate();

  const vaultAuthSeeds = [Buffer.from('auth'), vaultState.publicKey.toBuffer()];
  const vaultAuth = PublicKey.findProgramAddressSync(
    vaultAuthSeeds,
    program.programId
  )[0];

  const vaultSeeds = [Buffer.from('vault'), vaultAuth.toBuffer()];
  const vaultPda = PublicKey.findProgramAddressSync(
    vaultSeeds,
    program.programId
  )[0];

  it('airdrop tokens', async () => {
    const txhash = await provider.connection.requestAirdrop(
      keypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );
  });

  it('Is initialized!', async () => {
    const txhash = await program.methods
      .initialize()
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vaultAuth,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair, vaultState])
      .rpc();
  });
});

/*


// Create a devnet connection

const vaultAuthSeeds = [Buffer.from('auth'), vaultState.publicKey.toBuffer()];
const vaultAuth = PublicKey.findProgramAddressSync(
  vaultAuthSeeds,
  program.programId
)[0];

const vaultSeeds = [Buffer.from('vault'), vaultAuth.toBuffer()];
const vaultPda = PublicKey.findProgramAddressSync(
  vaultSeeds,
  program.programId
)[0];

(async () => {
  try {
    const txhash = await program.methods
      .initialize()
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vaultAuth,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair, vaultState])
      .rpc();
    console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
*/
