import { Connection, Keypair, SystemProgram, PublicKey } from '@solana/web3.js';
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
} from '@project-serum/anchor';
import { WbaPrereq, IDL } from '../programs/wba_prereq';
import wallet from '../wba-wallet.json';
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection('https://api.devnet.solana.com');
const mint = new PublicKey('4XhveW32K81i9jNZ57xLVUEX3m2HguGM1Yz7mEdAHH9K');

(async () => {
  try {
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    console.log(tokenAccount.address.toString());

    const txSig = await mintTo(
      connection,
      keypair,
      mint,
      tokenAccount.address,
      keypair,
      1000000000
    );

    console.log(txSig);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
