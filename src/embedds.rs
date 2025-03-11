
use std::{fs, ops::Index, usize};

use rand::seq::index;
use serenity::{all::{ ChannelId, Colour, CreateAttachment, CreateEmbed, CreateMessage, Member, Message, Reaction, Ready, Timestamp}, async_trait, builder, model::colour, prelude::*};




pub async fn send_welcome(msg:Message, ctx:Context){
let embed = CreateEmbed::default()
    .title("Quantum")
    .description("Quantum is a technical Minecraft server, launched on 5/9/2024, running on version 1.18.2. While we focus on pushing the limits of the game, our real mission is to create a thriving, supportive family. At Quantum, we believe in growth, both as players and as people. We’re here to help you unlock your full potential, share knowledge, and inspire each other every step of the way. Join us, and together we’ll break boundaries, build lasting connections, and achieve something truly extraordinary.")
    .color(Colour::new(0x623092))
    .timestamp(Timestamp::now())
    .fields(vec![
        ("Invite Link", "https://discord.gg/huU3gjchxp", false),
        ("Apply", "If you wish to join our family check the apply channel \n https://discord.com/channels/1226653152917327922/1226732610642903070", false)
    ])
    .thumbnail("https://cdn.discordapp.com/attachments/1230960828551925850/1335615583076618310/icon-1.png?ex=67a0d071&is=679f7ef1&hm=d856588a39b6bc30955ccd75f4d3d7d921c6b0435e3c7727362fffb54498ecc9&");
    
let message = CreateMessage::default().add_embed(embed);
msg
    .channel_id
    .send_message(&ctx.http, message)
    .await.unwrap();
}

pub async fn send_info(msg:Message, ctx:Context){
    let embed = CreateEmbed::default()
        .title("Server Info")
        .color(Colour::new(0x623092))
        .fields(vec![
            ("Rules", "No duping of any items except: TNT, Gravity blocks, Carpet, String, Rails, Elytras, Sponges \n Don't use the /player command on actuall members \n Dont touch any farms, trenchers or similar without knowing how to use or repair them \n Name Carpet bots after what they are loading (for example, name the bot 13WideTrencher and not sex_on_a_trench or d4",false),
            ("Servers", "**IP:                      Server Version:  18.2 \n SMP:                 Andrey2006.go.ro:25565 \n CMP:                Andrey2006.go.ro:25566 \n Copy:               Andrey2006.go.ro:25567 \n 1.20.4 CMP:   Andrey2006.go.ro:25568 \n\n Seed:              -6042349440058724837** ",false),
        ]);

    let message = CreateMessage::default().add_embed(embed);
    msg.channel_id.send_message(&ctx.http, message).await.unwrap();
    
}

pub fn embedd_from_json(json: &str) -> CreateEmbed {
    // let json_string = fs::read_to_string(path).unwrap();
    let mut json_string = json.to_string();

    // for line in json_string.lines() {
    //     if line.contains("color") {
    //         let mut colorcode = line.split_once("#").unwrap().1;
    //         colorcode = colorcode.split_once("\"").unwrap().0;
    //         let index = json_string.find("color").unwrap();
    //         
    //     }
    // }
   
    for line in json_string.clone().lines()  {
        if line.contains("\"color\": ") {
            let mut colorcode = line.split_once("#").unwrap().1;
            colorcode = colorcode.split_once("\"").unwrap().0;
            let index = json_string.find("\"color\": ").unwrap();
            
            json_string.replace_range(index-1..index+20, format!("\"color\": {},",i64::from_str_radix(colorcode, 16).unwrap()).as_str());
        }

        if line.contains("\"timestamp\": "){
            let index = json_string.find("\"timestamp\": ").unwrap();
            json_string.insert_str(index+1, "nuh uh");
        }

    }


    let embedd:CreateEmbed = serde_json::from_str(&json_string).unwrap();
    embedd
}

pub async fn send_embedd(msg:Message, ctx: Context, em: CreateEmbed) {
    let message = CreateMessage::default().add_embed(em);

    msg.channel_id.send_message(&ctx.http, message).await.unwrap();

}
