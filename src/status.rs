use serde_json::Value;
use serenity::all::{Context, CreateEmbed, CreateEmbedFooter, CreateMessage, Message};




impl crate::Handler{
    pub async fn status(&self, ctx: Context, msg: Message) {
        let mut res = String::new();

       
        let embed = CreateEmbed::new()
        .color(6434962)
        .footer(CreateEmbedFooter::new("brought to you by Quantum").icon_url("https://cdn.discordapp.com/attachments/1307289010418352158/1307289081889427456/icon.png?ex=6739c355&is=673871d5&hm=0aca440320466c9a61054b91024ac7bf77a989ea4025cff5475f7a820c850f80&"))
        .field("test", "Hello my friend", false)
        .field("test2", "not inline lol", false);

        for ip in self.data.server_ip.iter() {
            let raw_json:Value = serde_json::from_str(&reqwest::get(format!("{}{}",crate::STATUS_URL,ip)).await.expect("status api call failed").text().await.unwrap()).expect("json parse failed");
            let name = raw_json["motd"]["clean"].to_string();
            res.push_str(&format!("### {}\nstatus: {}\nplayers: {}\n",&name[2..name.len()-2],if raw_json["online"]==true{"online"}else{"offline"},raw_json["players"]["online"]));
        }
                        
        
        msg.channel_id.send_message(ctx.http, CreateMessage::new().embed(embed)).await.unwrap();        
    }
}