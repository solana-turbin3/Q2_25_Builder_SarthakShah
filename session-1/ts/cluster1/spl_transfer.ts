import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../id.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("CEnCyT4hzGLjqfivmsHZ95wayDGghxzb3XkHtj5p5oaN");

// Recipient address
const to = new PublicKey("DHWMKjZKgRHrtpcZTYikFgCaxHQ7mFRXV7HSdFx6wjd9");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,
        )
        console.log(`Your ATA is: ${fromWallet.address.toBase58()}`);

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to,
        )
        console.log(`Your ATA is: ${toWallet.address.toBase58()}`);

        // Transfer the new token to the "toTokenAccount" we just created
        const transferTokens = await transfer(
            connection,
            keypair,
            fromWallet.address,
            toWallet.address,
            keypair,
            10
        )

        console.log("Transaction:", transferTokens);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

// Your ATA is: 2EKCMraDi5fCNX7JP6kXq4vZM4wQ8GPPD8hEm6KiuKAt
// Your ATA is: GttiwhCdh8PA9wLKyLFachmc8RdR27ZP6mwipCuEBkr7
// Transaction: MYEvqr9RPVEBsfTaop94Tspm766YWHHRj4wTyUvne69bARpuqiJ1u7obfTZwR5WZu5UjCvr4bbZmomVF1d7YDex