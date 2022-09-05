use std::env; // Access environment variables

use serenity::{
    async_trait, // For asynchronous features
    model::{channel::Message, gateway::Ready}, // 
    prelude::*, // Basics for serenity
};

const PING_MESSAGE: &str = "Yes, I work!";
const PING_COMMAND: &str = "g!ping";
const D_TOKEN: &str = "DISCORD_TOKEN";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // In the event of a message being sent, check if it matches a command, then respond accordingly

        if msg.content == PING_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, PING_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        // Lets us know the Handler is ready
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]                                                                  // Denotes the main function of async program
async fn main() {
    let token: String = env::var(D_TOKEN)          // Create token
        .expect("Expected a token in the environemtn");
    let intents: GatewayIntents = GatewayIntents::non_privileged();             // Unsure what intents are for
    
    let mut client: Client = Client::builder(&token, intents)      // Create a new discord client
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }
}
