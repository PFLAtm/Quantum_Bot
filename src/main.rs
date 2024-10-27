use std::{fs::File, io::Read};
use rand::Rng;
use serde::Deserialize;
use serde_json::Value;
use serde_with::chrono::TimeDelta;
use serenity::{all::{ ChannelId, CreateAttachment, CreateMessage, Member, Message, Reaction, Ready, Timestamp}, async_trait, prelude::*};




const JOKE_URL:&'static str = "https://v2.jokeapi.dev/joke/Programming,Miscellaneous,Pun?blacklistFlags=nsfw,religious,political,racist,sexist,explicit&format=txt";
const STATUS_URL:&'static str ="https://api.mcsrvstat.us/3/";
const SKIN_URL:&'static str= "https://mc-heads.net/avatar/";
const HELP:&'static str = 
"### commands

- echo: repeats the given argument
- joke: prints a (hopefully) funny joke
- help: shows this text
- timeout: puts user in 5min timeout
- new-ms: reminds CA that a new ms has to be build
- copypasta: prints random copypasta
- status: prints quantum server status
- avatar: prints the head of the specified player skin
";




#[derive(Deserialize)]
struct Data{
    verified_message_id: u64,
    verified_emoji: String,
    verified_role_id: u64,
    welcome_channel_id: u64,
    bot_permission_role_id: u64,
    rules_channel_id: u64,
    copypasta: Vec<String>,
    server_ip: Vec<String>,
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
            let args = &msg.content[msg.content.find(" ").unwrap_or_else(||cmd.len())+1..msg.content.len()];
            let has_permission = msg.author.has_role(ctx.http.clone(), msg.guild_id.unwrap(), self.data.bot_permission_role_id).await.unwrap(); 
            let is_bot = msg.author.bot;
            

            let execute = match cmd{
                "help" => Some(msg.channel_id.say(ctx.http, HELP.to_string())),

                "echo" if has_permission || is_bot => {
                    let res = if args==""{"no arguments found".to_string()} else {
                        if args.contains("@") {
                            "❌".to_string()

                        } else if args.contains(">") {
                            let mut rec = 0;
                            let mut res=String::from("something went wrong :(");
                            for (i,v) in args.as_bytes().iter().enumerate() {
                                if v == &b'>' {
                                    rec +=1;
                                }
                                if  rec >= 6{
                                    let (with_rec,no_rec) = args.split_at(i-1);
                                    res = with_rec.to_string();
                                    res.push_str(&no_rec.replace(">", ""));
                                    break;
                                } else {
                                    res = args.to_string();
                                }
                            }
                            res
                        } 
                        else {
                            args.to_string()
                        }
                        
                    };
                    Some(msg.channel_id.say(ctx.http, res))
                    
                },

                "echo" => Some(msg.channel_id.say(ctx.http,"insufficient permisions".to_string())),

                "new-ms" => Some(msg.channel_id.say(ctx.http,"<@1166088970279583874> when can we build new ms? \n -block on block".to_string())),

                "copypasta" => Some(msg.channel_id.say(ctx.http,self.data.copypasta[rand::thread_rng().gen_range(0..self.data.copypasta.len())].to_string())),

                "joke" => {
                    let joke = reqwest::get(JOKE_URL).await.expect("joke api call failed").text().await.unwrap();
                    Some(msg.channel_id.say(ctx.http,joke))
                },

                "verify-all" if has_permission => {
                    let members = msg.guild_id.unwrap().members(ctx.http.clone(), None, None).await.unwrap();
                    for member in members {
                        member.add_role(ctx.http.clone(), self.data.verified_role_id).await.expect("add role in loop failed");
                    }
                    Some(msg.channel_id.say(ctx.http,"done".to_string()))
                },

                "timeout" if !is_bot=> {
                    let time = Timestamp::now().checked_add_signed(TimeDelta::minutes(5)).unwrap().to_rfc3339();
                    msg.member(ctx.http.clone()).await.unwrap().disable_communication_until_datetime(ctx.http, Timestamp::parse(&time).unwrap()).await.unwrap();
                    None
                },

                "status" => {
                    let mut res = String::new();
                    for ip in self.data.server_ip.iter() {
                        let raw_json:Value = serde_json::from_str(&reqwest::get(format!("{}{}",STATUS_URL,ip)).await.expect("status api call failed").text().await.unwrap()).expect("json parse failed");
                        let name = raw_json["motd"]["clean"].to_string();
                        res.push_str(&format!("### {}\nstatus: {}\nplayers: {}\n",&name[2..name.len()-2],if raw_json["online"]==true{"online"}else{"offline"},raw_json["players"]["online"]));
                    }
                    Some(msg.channel_id.say(ctx.http, res))
                    
                },

                "avatar" => {
                    if args=="" {
                        Some(msg.channel_id.say(ctx.http,"no argument found".to_string()))
                    } else {
                        let file_data = reqwest::get(&format!("{}{}/64.png",SKIN_URL,args)).await.expect("skin api call failed").bytes().await.unwrap();
                        let builder = CreateMessage::new().add_file(CreateAttachment::bytes(file_data,"skin.png"));
                        msg.channel_id.send_message(ctx.http, builder).await.unwrap();
                        None
                    }
                },

                _ => Some(msg.channel_id.say(ctx.http,"unknown command".to_string())),
            };

            if let Some(e) = execute{
                e.await.unwrap();
            }        
        
        }
    }

    //todo: mc server api 
    //todo: mojang api

    async fn guild_member_addition(&self, ctx:Context, mem:Member){
        let greetings = vec!["Welcome to the quantic party ", "Welcome to Quantum "];
        ChannelId::new(self.data.welcome_channel_id).say(ctx.http, format!("{}{}! make sure to check the rules in <#{}>",greetings[rand::thread_rng().gen_range(0..greetings.len())],mem.mention(),self.data.rules_channel_id)).await.unwrap();
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is online!", ready.user.name);
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