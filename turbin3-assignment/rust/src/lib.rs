mod programs;

#[cfg(test)]

mod tests {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer, system_program};
    use solana_sdk;
    use solana_sdk::blake3::hash;
    use solana_sdk::{
        message::Message,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    use crate::programs::turbin3_prereq::{CompleteArgs, TurbinePrereqProgram, UpdateArgs};

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");

        // Get base58 private key
        let secret_key = bs58::encode(&kp.to_bytes()).into_string();
        println!("Private key (base58): {}", secret_key);

        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdop() {
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());
        // After that we can verify the singature, using the default implementation
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        let to_pubkey = Pubkey::from_str("4uockXKuNbtNvayaJp5czTDSj5PeF5ssaYFLxVziRzBx").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);

        // Get latest blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Get current balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        println!("Current balance: {} SOL", balance as f64 / 1_000_000_000.0);

        // Create a message to calculate fee
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // Calculate fee
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        println!("Transaction fee: {} lamports", fee);

        // Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => {
                println!("Success! Sent entire balance minus fee.");
                println!(
                    "Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                    signature
                );

                // Verify new balance is (close to) zero
                let new_balance = rpc_client
                    .get_balance(&keypair.pubkey())
                    .expect("Failed to get new balance");
                println!("New balance: {} lamports", new_balance);
            }
            Err(e) => println!("Failed to send transaction: {}", e),
        }
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        
        let prereq = TurbinePrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Create the complete instruction with proper arguments
        let args = CompleteArgs {
            github: b"Not-Sarthak".to_vec(),
        };
        
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
        
        let ix = TurbinePrereqProgram::complete_ix(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
        );
        
        let transaction = Transaction::new_signed_with_payer(
            &[ix],
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
            
        println!(
            "Success! Check out your TX here:
    https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
