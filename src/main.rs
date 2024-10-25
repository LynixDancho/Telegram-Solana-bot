use dotenv::dotenv;
use std::{env, fmt::Error, str::FromStr};
use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};
use teloxide::{prelude::*,  utils::command::BotCommands};
use solana_client::rpc_client::RpcClient;
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
    CreateWallet,
    CheckBalance(String),

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
