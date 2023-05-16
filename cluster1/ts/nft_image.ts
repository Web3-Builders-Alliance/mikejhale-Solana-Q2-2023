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
//const vaultState = Keypair.generate();

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
    const image = await readFile('./images/rug-1.png');
    const metaplexImage = toMetaplexFile(image, 'rug1.png');
    const uri = await metaplex.storage().upload(metaplexImage);

    console.log(uri);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
