use dotenv::dotenv;
use std::{env,  str::FromStr};
use solana_sdk::{bs58, pubkey::{self, Pubkey}, signature::Keypair, signer::{keypair, Signer}};
use teloxide::{prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup}, utils::command::BotCommands};
use solana_client::{  rpc_client::RpcClient};
 
use std::sync::atomic::{AtomicUsize, Ordering};  
 use rayon::iter::{IntoParallelIterator,ParallelIterator};
 use teloxide::types::InputFile;
use qrcode::QrCode;
use image::Luma;
 
  #[tokio::main]
 async fn main(){
    dotenv().ok();
    match env::var("TELOXIDE_TOKEN"){
        Ok(token)=> println!("gg wp {}", token),
        Err(e) => println!("the error is : {}", e)


    }

   log::info!("starting ze bot");
  let  bot = Bot::from_env();
 Commands::repl(bot,commandsto_create_asolana_wallet_callit_asolana_project_hahah ).await;



}



 #[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Commands{
     Help, 
    #[command(description = "Creates a Wallet for you ")]

    CreateWallet,
    #[command(description = "u check ur balance with this provide the public key .")]

    CheckBalance(String),
    #[command(description = "Gives u the QrCode proviede the public key")]

    Deposit(String),
    #[command(description = "Create a Custom Wallet")]

    CustomWallet(String)


}

async fn commandsto_create_asolana_wallet_callit_asolana_project_hahah(bot:Bot,msg:Message,cmd:Commands) -> ResponseResult<()>{
    
    match cmd {
        Commands::CreateWallet => {
          let (prv_key,pub_key)    = creating_a_wallet().await;
            bot.send_message(msg.chat.id, format!("This is ur Public key: {}", pub_key) ).await?;
            bot.send_message(msg.chat.id, format!("This is ur Private key :{}", prv_key) ).await?;
            

        },
        Commands::Help => {
            bot.send_message(msg.chat.id, Commands::descriptions().to_string()).await?;

        },
        Commands::CheckBalance(pub_key)=>{
            checking_balance(bot,pub_key,msg).await?;
        },
        Commands::Deposit(key)=> {
            send_deposit_info(bot,msg,&key).await?;
        
        },
        Commands::CustomWallet(prefix) =>{
            bot.send_message(msg.chat.id, format!("This will  take time  Unless you're lucky") ).await?;
            bot.send_dice(msg.chat.id).await?;

           let test = custom_wallet( prefix,bot.clone(),msg.clone()).await;
          match test{
            Some(wallet) =>{ 
                let prv_key = bs58::encode(wallet.to_bytes()).into_string();
     
                bot.send_message(msg.chat.id, format!("This is ur Public key: {}", wallet.pubkey()) ).await?;
                bot.send_message(msg.chat.id, format!("This is ur Private key :{}", prv_key) ).await?;
     

            } 
            None =>{
                bot.send_message(msg.chat.id, "no key like that my man ").await?;
            }
          }
           
        }

        
    }
    Ok(())

} 
async fn creating_a_wallet() -> (String, String){
    let key = Keypair::new();
    let pub_key = key.pubkey().to_string();
    let prv_key = bs58::encode(key.to_bytes()).into_string();

(prv_key,pub_key)
}
async fn checking_balance(bot: Bot, pub_key: String, msg: Message) -> ResponseResult<()> {
    let client: RpcClient = RpcClient::new("https://api.devnet.solana.com");
    let pubkey = Pubkey::from_str(&pub_key).expect("Invalid public key");

    let response = match client.get_balance(&pubkey) {
        Ok(balance) => format!("Your balance is: {} lamports ", balance),
        Err(e) => format!("Error occurred: {}", e),
    };

    bot.send_message(msg.chat.id, response).await?; // Await the message sending
    Ok(())
}
async fn send_deposit_info(bot: Bot, msg: Message,key:&str) -> ResponseResult<()> {
    let wallet_address = key;
    
     
    let code = QrCode::new(wallet_address).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("qrcode.png").unwrap();   

    bot.send_photo(msg.chat.id, InputFile::file("qrcode.png"))
        .caption(format!("Deposit to this wallet:\n{}", wallet_address))
        .await?;
    Ok(())
}
// alright first i need to make a function that checks the authority and legitemacy of a token Alright htey will type the address and i check that shit 

async fn custom_wallet(prefix:String,bot: Bot,msg: Message) ->Option< Keypair>{
    let start = std::time::Instant::now();
    let attempts = AtomicUsize::new(0);
    let max_attempts = 1_000_000;
      for attempt  in 0..max_attempts{
         let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();
        attempts.fetch_add(1, Ordering::Relaxed);
        if attempt > 0 && attempt % 100_000 == 0 {
            let progress_message = format!("Attempted {} times without a match", attempt);
            bot.send_message(msg.chat.id, progress_message).await.ok()?;
        }

         if pubkey.starts_with(&prefix){
            let duration = start.elapsed();
            println!("Found matching wallet in {} attempts and {:?} seconds", attempts.load(Ordering::Relaxed), duration);
            return Some(keypair); 
        }

        
    }
    println!("Failed to find matching wallet within {} attempts", max_attempts);
    None
    
    
//     let wallet = (0..max_attempts).into_par_iter().find_map_any(|_| {
//           let keypair = Keypair::new();
//          let pub_key = keypair.pubkey().to_string();

//         attempts.fetch_add(1, Ordering::Relaxed);

//         if pub_key.starts_with(&prefix) {
//             Some(keypair)
//         } else {
//             None
//         }
//     }).expect("Failed to find matching wallet");

 
//  wallet

}



