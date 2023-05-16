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
import {
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { WbaVault, IDL } from './wba_vault';
import wallet from '../../wba-wallet.json';
import { exit } from 'process';

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
const connection = new Connection('https://api.devnet.solana.com');

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
const mint = new PublicKey('4XhveW32K81i9jNZ57xLVUEX3m2HguGM1Yz7mEdAHH9K');

(async () => {
  try {
    const ownerTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    const vaultTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      vaultAuth,
      true
    );

    console.log(vaultTokenAccount.address);

    const txhash = await program.methods
      .depositSpl(new BN(1000000))
      .accounts({
        owner: keypair.publicKey,
        ownerAta: ownerTokenAccount.address,
        vaultState: vaultState,
        vaultAuth: vaultAuth,
        vaultAta: vaultTokenAccount.address,
        tokenMint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
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
