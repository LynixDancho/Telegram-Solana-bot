use dotenv::dotenv;
use std::{env, str::FromStr, sync::Arc, thread::sleep, time::Duration};
use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};
use teloxide::{ prelude::*, types::InputFile, utils::command::BotCommands};
use solana_client::rpc_client::RpcClient;
use tokio::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use qrcode::QrCode;
use image::Luma;
use teloxide::types::{KeyboardButton,KeyboardMarkup   };
use dexscreener;
 
#[tokio::main]
async fn main() {
    dotenv().ok();
    match env::var("TELOXIDE_TOKEN") {
        Ok(token) => println!("Bot token found: {}", token),
        Err(e) => println!("Error loading token: {}", e),
    }

    log::info!("Starting the bot");
    let bot = Bot::from_env();
    let clientt: dexscreener::Client = dexscreener::Client::new();
    let is_running = Arc::new(Mutex::new(false));

    Commands::repl(bot.clone(), move |bot:Bot, msg: Message, cmd: Commands| {
        let client = clientt.clone();
        let is_running = Arc::clone(&is_running);
        async move {
            let keyboard = KeyboardMarkup::new(vec![
                vec![
                    KeyboardButton::new("/help"),
                    KeyboardButton::new("/createwallet"),
                
                ],
            ])
            .resize_keyboard();
    
            bot.send_message(msg.chat.id, "."  )
                .reply_markup(keyboard)
                .disable_notification(true)
                .await?;
            
             commandsto_create_asolana_wallet_callit_asolana_project_hahah(bot, msg, cmd, is_running,client).await
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
    #[command(description = "Check the token worth ")]
    CheckToken(String),
    #[command(description = "Stop generating custom wallets")]
    Stop,
}














async fn commandsto_create_asolana_wallet_callit_asolana_project_hahah(
    bot: Bot,
    msg: Message,
    cmd: Commands,
    is_running: Arc<Mutex<bool>>,
    client:dexscreener::Client
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
                *running = false; 
                bot.send_message(msg.chat.id, "Wallet generation has been stopped.").await?;
            } else {
                bot.send_message(msg.chat.id, "No wallet generation in progress.").await?;
            }
        }
        Commands::CheckToken(string)=>{
            match get_token_mint(&string.clone()) {
                Ok((freeze_authority, mint_authority)) => {
                    println!("Mint Authority: {}", mint_authority);
                    println!("Freeze Authority: {}", freeze_authority);
                }
                Err(err) => {
                    println!("Error fetching mint details: {}", err);
                }
            }            
            let token = vec![String::from(string)];
           match client.tokens(token).await {
                Ok(response) =>{
                    if let Some(pairs) = response.pairs {
                        for pair in pairs {
                             
                           bot.send_message(msg.chat.id,format!("Token Bought in  1 hour : {:?}", pair.txns.h1.buys )).await.ok() ;
                           bot.send_message(msg.chat.id,format!("Token Sold in  1 hour : {:?}", pair.txns.h1.sells )).await.ok() ; 
 
                           bot.send_message(msg.chat.id,format!("Token Bought in  5 minutes :   {:?}", pair.txns.m5.buys )).await.ok() ; 
                            bot.send_message(msg.chat.id,format!("Token Sold in  5 minutes :  {:?}", pair.txns.m5.sells )).await.ok() ; 

                           bot.send_message(msg.chat.id, format!(
                            "Liquidity: USD: {:.2}, Base: {:.2}, Quote: {:.4}", 
                            pair.liquidity.clone().unwrap().usd, 
                            pair.liquidity.clone().unwrap().base,
                            pair.liquidity.clone().unwrap().quote,
                            
                          )).await.ok();                           bot.send_message(msg.chat.id,format!("Last Price: {:?}", pair.price_usd.unwrap()) ).await.ok() ;  

                       
                        }
                    } else {
                        bot.send_message(msg.chat.id,"No pairs found for this token.").await.ok();
                    }


                }
                Err(error) =>{
                    bot.send_message(msg.chat.id, format!("Error Has Occured make sure u have the right token {}",error)).await?;
                }



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
    let client: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com");
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
     let mut attempt = 1;
    loop{
        // Check if `is_running` is set to false, exit early if so
        if !*is_running.lock().await {
            bot.send_message(msg.chat.id, "Wallet generation stopped.").await.ok();
            break;
        }

        let keypair = Box::new(Keypair::new());
        let pubkey = keypair.pubkey().to_string();
        attempts.fetch_add(1, Ordering::Relaxed);
        attempt+=1;

        if attempt > 0 && attempt % 100_000 == 0 {
            let progress_message = format!("Attempted {attempt} times without a match");
            bot.send_message(msg.chat.id, progress_message).await.ok();
            tokio::time::sleep(Duration::from_secs(2)).await;
            
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

 }

fn get_token_mint(mint_address: &str) -> Result<(Pubkey,Pubkey), Box<dyn std::error::Error>> {
        let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com");

        let mint_pubkey = Pubkey::from_str(mint_address)?;
   
        let account_info = rpc_client.get_account(&mint_pubkey)?;
   
        if account_info.data.len() < 82 {
           return Err("Account data is not valid for a token mint".into());
       }
   
        let mint_authority_bytes = &account_info.data[4..36];
       let mint_authority = Pubkey::new_from_array(mint_authority_bytes.try_into()?);
   
        let freeze_authority_bytes = &account_info.data[68..100];
       let freeze_authority = Pubkey::new_from_array(freeze_authority_bytes.try_into()?);
   
        println!("Mint Authority: {}", mint_authority);
       println!("Freeze Authority: {}", freeze_authority);
   
       Ok((freeze_authority,mint_authority))
}









 
