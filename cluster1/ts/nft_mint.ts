import { Commitment, Connection, Keypair } from '@solana/web3.js';
import wallet from '../../wba-wallet.json';
import {
  Metaplex,
  keypairIdentity,
  bundlrStorage,
  toMetaplexFile,
  PublicKey,
} from '@metaplex-foundation/js';
import { readFile } from 'fs/promises';
import { KeyObject } from 'crypto';

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection('https://api.devnet.solana.com', commitment);

const metadataUri =
  'https://arweave.net/Mn5YvYO3Y-Cyjm8_OV-WeXs8JaYrB_2TTzq64kuN7ms';

const metaplex = Metaplex.make(connection)
  .use(keypairIdentity(keypair))
  .use(
    bundlrStorage({
      address: 'https://devnet.bundlr.network',
      providerUrl: 'https://api.devnet.solana.com',
      timeout: 60000,
    })
  );

(async () => {
  try {
    const mint = await metaplex.nfts().create({
      uri: metadataUri,
      name: 'Mike Rug',
      symbol: 'MRUG',
      creators: [
        {
          address: keypair.publicKey,
          share: 100,
        },
      ],
      sellerFeeBasisPoints: 100,
      isMutable: true,
    });

    console.log(mint.nft.address);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
