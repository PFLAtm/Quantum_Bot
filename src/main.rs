use std::{fs::File, io::Read};
use serenity::{all::{ Message, Reaction, Ready}, async_trait, prelude::*};



struct Handler;
#[async_trait]
impl EventHandler for Handler{  
    async fn message(&self, ctx:Context, msg:Message){
        if msg.content == "!nono"{
            msg.channel_id.say(ctx.http, "raaaaaaaa").await.unwrap();             
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn reaction_add (&self, ctx:Context, rct:Reaction) {
       if rct.message_id == 1299380119848353924 && rct.emoji.unicode_eq("✅"){
        rct.member.unwrap().add_role(ctx.http, 1299385080061366282).await.unwrap();
       } 
    }
}

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut token = String::new();
    File::open("token.txt").expect("no token file").read_to_string(&mut token).expect("file read failed");

    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("building client failed");

    if let Err(e) = client.start().await {
        println!("reason for Error:{e}");
    }
}