import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from '@solana/web3.js';
import wallet from '../../wba-wallet.json';
import { getOrCreateAssociatedTokenAccount, transfer } from '@solana/spl-token';

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection('https://api.devnet.solana.com', commitment);

// Mint address
const mint = new PublicKey('4XhveW32K81i9jNZ57xLVUEX3m2HguGM1Yz7mEdAHH9K');

// Recipient address
const to = new PublicKey('3JnfMbmTtQQd6bdQ6ahNKYKdq4ujgMNbGaQpWZwQYqWh');

(async () => {
  try {
    const from_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    const to_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );

    // Transfer the new token to the "toTokenAccount" we just created
    const tx = transfer(
      connection,
      keypair,
      from_ata.address,
      to_ata.address,
      keypair.publicKey,
      1000000
    );
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
