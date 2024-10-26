use dotenv::dotenv;
use std::{env,  str::FromStr};
use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};
use teloxide::{prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup}, utils::command::BotCommands};
use solana_client::rpc_client::RpcClient;
use url::Url;
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

    Deposit(String)

}

async fn commandsto_create_asolana_wallet_callit_asolana_project_hahah(bot:Bot,msg:Message,cmd:Commands) -> ResponseResult<()>{
    let url = Url::parse("https://www.youtube.com/watch?v=J87pJrxvJ5E").expect("Invalid URL");

    // let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
    //     "Open Phantom",
    //     url,
    // )]]); 
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
    
    // Generate QR code
    let code = QrCode::new(wallet_address).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("qrcode.png").unwrap();  // Save the QR code image locally

    // Send the QR code and wallet address to the user
    bot.send_photo(msg.chat.id, InputFile::file("qrcode.png"))
        .caption(format!("Deposit to this wallet:\n{}", wallet_address))
        .await?;
    Ok(())
}