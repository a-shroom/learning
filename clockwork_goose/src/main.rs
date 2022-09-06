pub mod hangman;

use std::path::Path;
use std::env; // Access environment variables

use serenity::{
    async_trait, // For asynchronous features
    model::{channel::Message, gateway::Ready}, 
    prelude::*, // Basics for serenity
    framework::standard::{
        macros::{command, group},
        Args,
        CommandResult,
        StandardFramework,
    }
};

use hangman::make_guess;

const D_TOKEN: &str = "DISCORD_TOKEN";
const FILENAME: &str = "hangman.txt";

#[command]
async fn guess(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let path: &Path = Path::new(FILENAME);
    let res: String = make_guess(&path, args.single::<String>().unwrap());

    msg.channel_id.say(&ctx.http, res).await?;

    Ok(())
}

#[command]
async fn honk(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Honk!").await?;

    Ok(())
}

#[group]
#[commands(guess, honk)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        // Lets us know the Handler is ready
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]                                                                  // Denotes the main function of async program
async fn main() {
    // Create a framework (used for command parsing)
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("g!"))
        .group(&GENERAL_GROUP);

    // Initiate bot with a token and declaring intents (aka what it can do)
    let token: String = env::var(D_TOKEN)          // Create token
        .expect("Expected a token in the environemt");
    let intents: GatewayIntents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Initiate and start client
    let mut client: Client = Client::builder(&token, intents)      // Create a new discord client
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }
}
