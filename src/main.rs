#![feature(drain_filter)]

pub mod bot_error;
pub mod utils;
pub mod stat;
mod application_commands;
mod leaderboard;
mod scheduled_leaderboards;
#[cfg(test)]
mod tests;

use std::env;

use serenity::{
    async_trait,
    http::client::Http,
    model::{
        // id::GuildId,
        gateway::Ready,
        interactions::{
            ApplicationCommand,
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
    leaderboard::{ parse_leaderboard_args, get_leaderboard, create_leaderboard_embed },
    stat::{ get_stat, parse_stat_args, create_stat_embed },
    scheduled_leaderboards::schedule_leaderboards,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(InteractionData::ApplicationCommand(ref command)) = interaction.data {
            match command.name.as_str() {
                "stat" => {
                    let args = parse_stat_args(&command.options);

                    let stat_result = get_stat(&args.player, &args.stat_type, &args.stat_value).await;

                    if let Err(e) = interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    match stat_result {
                                        Ok(stat) => message.create_embed(|e| 
                                            create_stat_embed(
                                                stat.value, args.player, stat.uuid,
                                                args.stat_type, args.stat_value, e
                                            )
                                        ),
                                        Err(e) => message.content(match e {
                                            BotError::Error(e) => e,
                                            BotError::ReqwestError(e) => e.to_string(),
                                            BotError::SerenityError(e) => e.to_string(),
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
                    let args = parse_leaderboard_args(&command.options);

                    let leaderboard_result = get_leaderboard(&args.stat_type, &args.stat_value, args.limit).await;

                    if let Err(e) = interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    match leaderboard_result {
                                        Ok(leaderboard) => message.create_embed(|e|
                                            create_leaderboard_embed(
                                                leaderboard, &args.stat_type, &args.stat_value, e
                                            )
                                        ),
                                        Err(e) => message.content(match e {
                                            BotError::Error(e) => e,
                                            BotError::ReqwestError(e) => e.to_string(),
                                            BotError::SerenityError(e) => e.to_string(),
                                        })
                                    }
                                })
                        })
                    .await {
                        println!("Cannot respond to slash command: {}", e)
                    }
                },
                _ => {},//"not implemented :(".to_string(),
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);

        let _ = ApplicationCommand::create_global_application_commands(&ctx.http, |commands| {
            create_application_commands(commands)
        })
        .await;

        // println!("I now have the following slash commands: {:?}", commands);
        // let _ = GuildId(669507869791748117)
        //     .create_application_commands(&ctx.http, |commands| {
        //         create_application_commands(commands)
        //     })
        //     .await;
        // let cmd = GuildId(587898993917427713)
        //     .create_application_commands(&ctx.http, |commands| {
        //         create_application_commands(commands)
        //     })
        //     .await;
        //
        // println!("I created the following guild command: {:#?}", cmd);
        //
        // Start the scheduled leaderboards update
        // let _leaderboards_future = schedule_leaderboards(&ctx.http);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {

    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "post_a_random_message" {
            msg.channel_id.say(&ctx.http, "random message").await.unwrap();
        } 
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in env");

    let http = Http::new_with_token(&token);

    let application_id = env::var("APPLICATION_ID")
        .expect("Expected APPLICATION_ID in env")
        .parse()
        .expect("APPLICATION_ID isn't valid");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // Start the scheduled leaderboards updates
    let _ = schedule_leaderboards(&http).await;

    if let Err(e) = client.start().await {
        println!("Client error: {}", e);
    }
}
