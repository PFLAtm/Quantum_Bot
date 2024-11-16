use std::{fs::OpenOptions, io::{Read, Write}, process};
use rand::Rng;
use serde::Deserialize;
use serde_with::chrono::TimeDelta;
use serenity::{all::{ ChannelId, CreateAttachment, CreateMessage, Member, Message, Reaction, Ready, Timestamp}, async_trait, prelude::*};


mod status;



const JOKE_URL:&'static str = "https://v2.jokeapi.dev/joke/Programming,Miscellaneous,Pun?blacklistFlags=nsfw,religious,political,racist,sexist,explicit&format=txt";
const STATUS_URL:&'static str ="https://api.mcsrvstat.us/3/";
const SKIN_URL:&'static str = "https://mc-heads.net/avatar/";
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

const DEFAULT_DATA:&'static str =
"#put Strings in double quotes
verified_message_id = 123456789 
verified_emoji = ✏️
verified_role_id = 123456789
welcome_channel_id = 123456789
bot_permission_role_id = 123456789
rules_channel_id = 123456789
copypasta = [copypasta1,copypaste2,...]
server_ip = [server_ip1,server_ip2,...]
greetings = [greeting1,greeting2,...]";




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
    greetings: Vec<String>,
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
                    self.status(ctx,msg).await;
                    None
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

                "stop" if has_permission => {
                    println!("stopping...");
                    process::exit(0)
                },
                "" => None,

                _ => Some(msg.channel_id.say(ctx.http,"unknown command".to_string())),
            };

            if let Some(e) = execute{
                e.await.unwrap();
            }        
        
        }
    }

    
    //todo: mojang api

    async fn guild_member_addition(&self, ctx:Context, mem:Member){
        ChannelId::new(self.data.welcome_channel_id).say(ctx.http, format!("{}{}! make sure to check the rules in <#{}>",self.data.greetings[rand::thread_rng().gen_range(0..self.data.greetings.len())],mem.mention(),self.data.rules_channel_id)).await.unwrap();
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
    
    //token loading
    let mut token_file = OpenOptions::new().read(true).write(true).create(true).open("token.txt").expect("token.txt open failed");
    let mut token = String::new();
    
    token_file.read_to_string(&mut token).expect("token read failed");
    if token == ""{
        println!("please put bot token in token.txt");
        process::exit(1);
    };

    //data loading
    let mut data_file = OpenOptions::new().create(true).write(true).read(true).open("data.toml").expect("data.toml open failed");
    let mut data_string = String::new();

    data_file.read_to_string(&mut data_string).expect("data read failed");
    if data_string=="" {
        data_file.write_all(DEFAULT_DATA.as_bytes()).unwrap();
        data_file.read_to_string(&mut data_string).expect("data read failed");
    }

    let data:Data = toml::from_str(&data_string).unwrap_or_else(|_|{
        let amount = DEFAULT_DATA.lines().enumerate().last().unwrap().0-(DEFAULT_DATA.lines().enumerate().last().unwrap().0-data_string.lines().enumerate().last().unwrap().0);
        let mut lines = DEFAULT_DATA.lines();
        for _ in 0..amount+1 {
            lines.next();
        }
       
        let mut write_str = String::new();
        lines.for_each(|s|{write_str.push_str(&format!("\n{s}"));});
        
        data_file.write_all(format!("\n{}",write_str).as_bytes()).expect("data.toml update failed");
        println!("please update data.toml");
        process::exit(1);
    });

    //client loop
    let mut client = Client::builder(token, intents).event_handler(Handler::new(data)).await.expect("building client failed");

    if let Err(e) = client.start().await {
        println!("reason for Error:{e}");
    }
}