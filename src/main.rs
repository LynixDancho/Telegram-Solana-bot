use dotenv::dotenv;
use std::{env, str::FromStr, sync::Arc};
use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
use solana_client::rpc_client::RpcClient;
use tokio::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use qrcode::QrCode;
use image::Luma;

#[tokio::main]
async fn main() {
    dotenv().ok();
    match env::var("TELOXIDE_TOKEN") {
        Ok(token) => println!("Bot token found: {}", token),
        Err(e) => println!("Error loading token: {}", e),
    }

    log::info!("Starting the bot");
    let bot = Bot::from_env();
    let is_running = Arc::new(Mutex::new(false));

    Commands::repl(bot, move |bot, msg, cmd| {
        let is_running = Arc::clone(&is_running);
        async move {
            commandsto_create_asolana_wallet_callit_asolana_project_hahah(bot, msg, cmd, is_running).await
        }
    })
    .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Commands {
    Help,
    #[command(description = "Creates a wallet for you")]
    CreateWallet,
    #[command(description = "Check your balance with this command, provide the public key.")]
    CheckBalance(String),
    #[command(description = "Provides a QR code for the public key")]
    Deposit(String),
    #[command(description = "Create a custom wallet")]
    CustomWallet(String),
    #[command(description = "Stop generating custom wallets")]
    Stop,
}

async fn commandsto_create_asolana_wallet_callit_asolana_project_hahah(
    bot: Bot,
    msg: Message,
    cmd: Commands,
    is_running: Arc<Mutex<bool>>,
) -> ResponseResult<()> {
    match cmd {
        Commands::CreateWallet => {
            let (prv_key, pub_key) = creating_a_wallet().await;
            bot.send_message(msg.chat.id, format!("This is your public key: {}", pub_key)).await?;
            bot.send_message(msg.chat.id, format!("This is your private key: {}", prv_key)).await?;
        }
        Commands::Help => {
            bot.send_message(msg.chat.id, Commands::descriptions().to_string()).await?;
        }
        Commands::CheckBalance(pub_key) => {
            checking_balance(bot, pub_key, msg).await?;
        }
        Commands::Deposit(key) => {
            send_deposit_info(bot, msg, &key).await?;
        }
        Commands::CustomWallet(prefix) => {
            let mut running = is_running.lock().await;
            if !*running {
                *running = true;
                drop(running); // Release the lock

                bot.send_message(msg.chat.id, "This will take time unless you're lucky").await?;
                bot.send_dice(msg.chat.id).await?;

                let is_running = Arc::clone(&is_running);
                let bot = bot.clone();
                let msg = msg.clone();
                let prefix = prefix.clone();

                tokio::spawn(async move {
                    custom_wallet(prefix, bot, msg, is_running).await;
                });
            } else {
                bot.send_message(msg.chat.id, "Wallet generation is already in progress.").await?;
            }
        }
        Commands::Stop => {
            let mut running = is_running.lock().await;
            if *running {
                *running = false; // Stop the wallet generation
                bot.send_message(msg.chat.id, "Wallet generation has been stopped.").await?;
            } else {
                bot.send_message(msg.chat.id, "No wallet generation in progress.").await?;
            }
        }
    }
    Ok(())
}

async fn creating_a_wallet() -> (String, String) {
    let key = Keypair::new();
    let pub_key = key.pubkey().to_string();
    let prv_key = bs58::encode(key.to_bytes()).into_string();
    (prv_key, pub_key)
}

async fn checking_balance(bot: Bot, pub_key: String, msg: Message) -> ResponseResult<()> {
    let client: RpcClient = RpcClient::new("https://api.devnet.solana.com");
    let pubkey = Pubkey::from_str(&pub_key).expect("Invalid public key");

    let response = match client.get_balance(&pubkey) {
        Ok(balance) => format!("Your balance is: {} lamports", balance),
        Err(e) => format!("Error occurred: {}", e),
    };

    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}

async fn send_deposit_info(bot: Bot, msg: Message, key: &str) -> ResponseResult<()> {
    let wallet_address = key;

    let code = QrCode::new(wallet_address).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("qrcode.png").unwrap();

    bot.send_photo(msg.chat.id, InputFile::file("qrcode.png"))
        .caption(format!("Deposit to this wallet:\n{}", wallet_address))
        .await?;
    Ok(())
}

async fn custom_wallet(prefix: String, bot: Bot, msg: Message, is_running: Arc<Mutex<bool>>) {
    let start = std::time::Instant::now();
    let attempts = AtomicUsize::new(0);
    let max_attempts = 1_000_000;

    for attempt in 0..max_attempts {
        // Check if `is_running` is set to false, exit early if so
        if !*is_running.lock().await {
            bot.send_message(msg.chat.id, "Wallet generation stopped.").await.ok();
            break;
        }

        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();
        attempts.fetch_add(1, Ordering::Relaxed);

        if attempt > 0 && attempt % 100_000 == 0 {
            let progress_message = format!("Attempted {} times without a match", attempt);
            bot.send_message(msg.chat.id, progress_message).await.ok();
        }

        if pubkey.starts_with(&prefix) {
            let duration = start.elapsed();
            println!("Found matching wallet in {} attempts and {:?} seconds", attempts.load(Ordering::Relaxed), duration);
            let prv_key = bs58::encode(keypair.to_bytes()).into_string();
            bot.send_message(msg.chat.id, format!("This is your public key: {}", pubkey)).await.ok();
            bot.send_message(msg.chat.id, format!("This is your private key: {}", prv_key)).await.ok();
            break;
        }
    }

    println!("Finished generating wallets or stopped.");
}
