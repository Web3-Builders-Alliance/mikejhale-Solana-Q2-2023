import { Commitment, Connection, Keypair } from '@solana/web3.js';
import wallet from '../../wba-wallet.json';
import {
  Metaplex,
  keypairIdentity,
  bundlrStorage,
  toMetaplexFile,
} from '@metaplex-foundation/js';
import { readFile } from 'fs/promises';

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection('https://api.devnet.solana.com', commitment);

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
    const { uri } = await metaplex.nfts().uploadMetadata({
      name: 'Mike Rug',
      symbol: 'MRUG',
      description: 'A fine rug',
      image: 'https://arweave.net/TtIqTwAqV24RHitVkvvbOfAmhsPK11jU1esZ--UhcoA',
      attributes: [
        { trait_type: 'name', value: 'Rugenstiein' },
        { trait_type: 'style', value: 'Shabby chic' },
      ],
      properties: {
        files: [
          {
            type: 'image/png',
            uri: 'https://arweave.net/TtIqTwAqV24RHitVkvvbOfAmhsPK11jU1esZ--UhcoA',
          },
        ],
      },
      creators: [
        {
          address: '5cFh8kSa7NfGyYQgFsLXSWqsmr36KS2pvw365rtUD5W7',
          share: 100,
        },
      ],
    });

    console.log(uri);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
