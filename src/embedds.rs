use std::thread::Builder;

use serenity::{all::{ ChannelId, Colour, CreateAttachment, CreateEmbed, CreateMessage, Member, Message, Reaction, Ready, Timestamp}, async_trait, builder, prelude::*};

pub async fn send_welcome(msg:Message, ctx:Context){
let embed = CreateEmbed::default()
    .title("Quantum")
    .description("Quantum is a technical Minecraft server, launched on 5/9/2024, running on version 1.18.2. While we focus on pushing the limits of the game, our real mission is to create a thriving, supportive family. At Quantum, we believe in growth, both as players and as people. We’re here to help you unlock your full potential, share knowledge, and inspire each other every step of the way. Join us, and together we’ll break boundaries, build lasting connections, and achieve something truly extraordinary.")
    .color(Colour::new(0x00b0f4))
    .timestamp(Timestamp::now())
    .fields(vec![
        ("Invite Link", "https://discord.gg/huU3gjchxp", false),
        ("Apply", "If you wish to join our family check the apply channel https://discord.com/channels/1226653152917327922/1226732610642903070", false)
    ])
    .thumbnail("https://cdn.discordapp.com/attachments/1230960828551925850/1335615583076618310/icon-1.png?ex=67a0d071&is=679f7ef1&hm=d856588a39b6bc30955ccd75f4d3d7d921c6b0435e3c7727362fffb54498ecc9&");
    
let message = CreateMessage::default().add_embed(embed);
let msg = msg
    .channel_id
    .send_message(&ctx.http, message)
    .await.unwrap();
}
