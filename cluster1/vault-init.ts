import { Connection, Keypair, SystemProgram, PublicKey } from '@solana/web3.js';
import { Program, Wallet, AnchorProvider, Address } from '@coral-xyz/anchor';
import { WbaVault, IDL } from './wba_vault';
import wallet from '../wba-wallet.json';

/*
-Load the IDL for the WBA Vault
-Initialize an Account with WBA Vault
-Deposit native Solana
-Withdraw native Solana
-Deposit your SPL token
-Withdraw SPL token
-Each successful interaction increases your WBA Score.
*/

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const vaultState = Keypair.generate();

// Create a devnet connection
const connection = new Connection('https://api.devnet.solana.com', commitment);

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: 'confirmed',
});

const program = new Program<WbaVault>(
  IDL,
  'D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o' as Address,
  provider
);

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
