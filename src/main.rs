use std::{fs::File, io::Read};
use serde::Deserialize;
use serenity::{all::{ ChannelId, Member, Message, Reaction, Ready}, async_trait, prelude::*};


const JOKE_URL:&'static str = "https://v2.jokeapi.dev/joke/Programming,Miscellaneous,Pun?blacklistFlags=nsfw,religious,political,racist,sexist,explicit&format=txt";
const HELP:&'static str = "### commands

- echo: repeats the given argument
- joke: prints a (hopefully) funny joke
- help: shows this text"
;


#[derive(Deserialize)]
struct Data{
    verified_message_id: u64,
    verified_emoji: String,
    verified_role_id: u64,
    welcome_channel_id: u64,
}

struct Handler{
    data:Data,
}

impl Handler {
    fn new(data: Data) -> Self{
        Handler {data}
    }
}
#[async_trait]
impl EventHandler for Handler{  
    //message commands: key symbol = >
    async fn message(&self, ctx:Context, msg:Message){
        if msg.content.starts_with(">"){
            let cmd = &msg.content[1..msg.content.find(" ").unwrap_or_else(||msg.content.len())];
            let args = &msg.content[msg.content.find(" ").unwrap_or_else(||cmd.len()+1)..msg.content.len()];
            

            let execute = match cmd{
                "help" => msg.channel_id.say(ctx.http, HELP.to_string()),
                "echo" => {
                    let res = if args==""{"no arguments found".to_string()} else {
                        if args.contains("@") {
                            "❌".to_string()

                        } else if args.contains(">") {
                            args.replace(">", "")
                        } 
                        else {
                            args.to_string()
                        }
                        
                    };
                    msg.channel_id.say(ctx.http, res)
                    
                },
                //"storage" => msg.channel_id.say(ctx.http, content),
                "copypasta" => msg.channel_id.say(ctx.http,"CA said no :(".to_string()),
                "joke" => {
                    let joke = reqwest::get(JOKE_URL).await.expect("joke api call failed").text().await.unwrap();
                    msg.channel_id.say(ctx.http,joke)
                },
                _ => msg.channel_id.say(ctx.http,"unknown command".to_string()),
            };

            execute.await.unwrap();
        
        
        }
    }

    async fn guild_member_addition(&self, ctx:Context, mem:Member){
        ChannelId::new(self.data.welcome_channel_id).say(ctx.http, format!("welcome {} to Quantum!",mem.nick.unwrap_or("".to_string()))).await.unwrap();
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn reaction_add (&self, ctx:Context, rct:Reaction) {
       if rct.message_id == self.data.verified_message_id && rct.emoji.unicode_eq(&self.data.verified_emoji){
        rct.member.unwrap().add_role(ctx.http, self.data.verified_role_id).await.unwrap();
       } 
    }
}

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::GUILD_MEMBERS;

    let mut token = String::new();
    let mut data_string = String::new();
    File::open("token.txt").expect("no token file").read_to_string(&mut token).expect("file read failed");
    File::open("data.toml").expect("no data toml file").read_to_string(&mut data_string).expect("file read failed");

    let data:Data = toml::from_str(&data_string).expect("deserilazation of data failed");

    let mut client = Client::builder(token, intents).event_handler(Handler::new(data)).await.expect("building client failed");

    if let Err(e) = client.start().await {
        println!("reason for Error:{e}");
    }
}