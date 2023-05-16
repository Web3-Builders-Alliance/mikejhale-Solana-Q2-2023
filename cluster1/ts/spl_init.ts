import { Connection, Keypair, SystemProgram, PublicKey } from '@solana/web3.js';
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
} from '@project-serum/anchor';
//import { WbaPrereq, IDL } from '../../programs/wba_prereq';
import wallet from '../../wba-wallet.json';
import { createMint } from '@solana/spl-token';

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection('https://api.devnet.solana.com');

(async () => {
  try {
    const mint = await createMint(
      connection,
      keypair,
      keypair.publicKey,
      null,
      6
    );

    console.log(mint.toBase58());
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
