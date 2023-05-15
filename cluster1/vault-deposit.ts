import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from '@coral-xyz/anchor';
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
//const vaultState = Keypair.generate();

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

const vaultState = new PublicKey(
  '6zCECf5sr529jSsURZt5aHFp7GK6LiZ6wCHh4W3t4uaR'
);
const vaultAuth = new PublicKey('82P4TPxUHhCfWA1v1FfTCGFvRJEsCYZvhWHZA7w9Z9SQ');
const vault = new PublicKey('2b7Hc4otHLX6JPuytZYcutxjNhheji7ANDZkN9R39CXQ');

(async () => {
  try {
    const txhash = await program.methods
      .deposit(new BN(0.1 * LAMPORTS_PER_SOL))
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState,
        vaultAuth: vaultAuth,
        vault: vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();
    console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
