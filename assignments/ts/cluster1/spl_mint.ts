import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../id.json"

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

const mint = new PublicKey("CEnCyT4hzGLjqfivmsHZ95wayDGghxzb3XkHtj5p5oaN");

(async () => {
    try {
        const ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,
        )
        console.log(`Your ATA is: ${ata.address.toBase58()}`);

        const mintTx = await mintTo(
            connection,
            keypair,
            mint,
            ata.address,
            keypair.publicKey,
            token_decimals,
        )
        console.log(`Your Mint TxID: ${mintTx}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()

// Your ATA is: 2EKCMraDi5fCNX7JP6kXq4vZM4wQ8GPPD8hEm6KiuKAt
// Your Mint TxID: 4P6r3xHQ8PfW1tCH7ubKhmKx7Ja6QPmHM9QJYrQcwcGbRraH2tCFRuBPWHyph88YnsLmVHytZDyMT137ooxnsBU8