pub mod bot_error;
pub mod utils;
pub mod stat;
mod application_commands;
mod leaderboard;

use std::env;

use serenity::{
    async_trait,
    model::{
        id::GuildId,
        gateway::Ready,
        interactions::{
            Interaction,
            InteractionResponseType,
            InteractionData,
        },
        event::ResumedEvent,
        channel::Message,
    },
    prelude::*,
};

use crate::{
    application_commands::create_application_commands,
    bot_error::BotError,
    utils::*,
    leaderboard::get_leaderboard,
    stat::{ get_stat, get_uuid_from_username },
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(InteractionData::ApplicationCommand(ref command)) = interaction.data {
            match command.name.as_str() {
                "stat" => {
                    let mut options_iter = command.options.iter();
                    let player = options_iter 
                        .find(|&x| x.name.as_str() == "player")
                        .unwrap()
                        .value.as_ref()
                        .unwrap()
                        .to_string()
                        .replace("\"", "");
                    let mut stat_type = options_iter
                        .find(|&x| x.name.as_str() == "stat-type")
                        .unwrap()
                        .value.as_ref()
                        .unwrap()
                        .to_string()
                        .replace("\"", "");
                    let mut stat_value = options_iter
                        .find(|&x| x.name.as_str() == "stat-value")
                        .unwrap()
                        .value.as_ref()
                        .unwrap()
                        .to_string()
                        .replace("\"", "");

                    let stat_result = get_stat(&player, &stat_type, &stat_value).await;

                    let uuid_res = get_uuid_from_username(player.clone()).await;

                    if let Err(e) = interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    match stat_result {
                                        Ok(stat) => {
                                            message.create_embed(|e| {
                                                e.title(&player);
                                                if let Ok(uuid) = uuid_res {
                                                    // e.image(format!("https://crafatar.com/avatars/{}", uuid));
                                                    e.thumbnail(format!("https://crafatar.com/avatars/{}", uuid));
                                                }
                                                make_ascii_titlecase(&mut stat_value);
                                                make_ascii_titlecase(&mut stat_type);

                                                if stat_value.chars().last().unwrap() != 's' {
                                                    stat_value = format!("{}s", stat_value);
                                                }
                                                let field_name = match stat_type.as_str() {
                                                    "custom" => stat_value.clone(),
                                                    "killed by" => format!("{} {}", stat_type, stat_value),
                                                    _ => format!("{} {}", stat_value, stat_type),
                                                };
                                                e.field(field_name, stat.value.to_string(), false);
                                                e.field("<:copper_ingot:863081302079963136> and text", ":copper_ingot: and text", false);

                                                e.color((200, 255, 0));

                                                e
                                            })
                                        }
                                        Err(e) => message.content(match e {
                                            BotError::Error(e) => e,
                                            BotError::ReqwestError(e) => e.to_string(),
                                        }),
                                    }
                                })
                        })
                    .await
                    {
                        println!("Cannot respond to slash command: {}", e)
                    }
                },
                "leaderboard" => {
                    let mut options_iter = command.options.iter();
                    let mut stat_type = options_iter
                        .find(|&x| x.name.as_str() == "stat-type")
                        .unwrap()
                        .value.as_ref()
                        .unwrap()
                        .to_string()
                        .replace("\"", "");
                    let mut stat_value = options_iter
                        .find(|&x| x.name.as_str() == "stat-value")
                        .unwrap()
                        .value.as_ref()
                        .unwrap()
                        .to_string()
                        .replace("\"", "");

                    let leaderboard_result = get_leaderboard(&stat_type, &stat_value, None).await;
                    println!("{:?}", leaderboard_result);
                },
                _ => {},//"not implemented :(".to_string(),
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);

        // let commands = ApplicationCommand::create_global_application_commands(&ctx.http, |commands| {
        // })
        // .await;

        // println!("I now have the following slash commands: {:?}", commands);
        let cmd = GuildId(669507869791748117)
            .create_application_commands(&ctx.http, |commands| {
                create_application_commands(commands)
            })
            .await;

        println!("I created the following guild command: {:#?}", cmd);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {

    }

    async fn message(&self, _: Context, _msg: Message) {

    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in env");

    let application_id = env::var("APPLICATION_ID")
        .expect("Expected APPLICATION_ID in env")
        .parse()
        .expect("APPLICATION_ID isn't valid");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {}", e);
    }
}
