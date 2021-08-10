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
        id::ChannelId,
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
    leaderboard::{ parse_leaderboard_args, get_leaderboard, create_leaderboard_embed },
    stat::{ get_stat, parse_stat_args, create_stat_embed },
    scheduled_leaderboards::schedule_leaderboards,
    utils::LEADERBOARDS_CHANNEL,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(InteractionData::ApplicationCommand(ref command)) = interaction.data {
            match command.name.as_str() {
                "stat" => {
                    let args = parse_stat_args(&command.options);

                    let stat_result = get_stat(&args.player, &args.stat_type, &args.stat_name).await;

                    if let Err(e) = interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    match stat_result {
                                        Ok(stat) => message.create_embed(|e| 
                                            create_stat_embed(
                                                stat.value, args.player, stat.uuid,
                                                args.stat_type, args.stat_name, e
                                            )
                                        ),
                                        Err(e) => message.content(e),
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

                    let leaderboard_result = get_leaderboard(&args.stat_type, &args.stat_name, args.limit).await;

                    if let Err(e) = interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    // println!("leaderboard_result = {:?}", leaderboard_result);
                                    match leaderboard_result {
                                        Ok(leaderboard) => message.create_embed(|e|
                                            create_leaderboard_embed(
                                                leaderboard, &args.stat_type, &args.stat_name, e
                                            )
                                        ),
                                        Err(e) => message.content(e)
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

        let _slash_commands = ApplicationCommand::create_global_application_commands(&ctx.http, |commands| {
            create_application_commands(commands)
        })
        .await
        .unwrap();

        /*
        let estilla = ctx.http.get_guild(587898993917427713).await.unwrap();
        let test_server = ctx.http.get_guild(669507869791748117).await.unwrap();

        for command in slash_commands.iter() {
            for o in command.options.iter() {
                println!("{}, {}", command.name, o.name);
                if &o.name == "stat-value" {
                    estilla.delete_application_command(&ctx.http, command.id).await.unwrap();
                    test_server.delete_application_command(&ctx.http, command.id).await.unwrap();
                    break;
                }
            }
        }

        let guild_commands = ctx.http.get_guild_application_commands(669507869791748117).await.unwrap();
        for command in guild_commands.iter() {
            for o in command.options.iter() {
                if &o.name == "stat-value" {
                    ctx.http.delete_guild_application_command(669507869791748117, *command.id.as_u64()).await.unwrap();
                }
            }
        }
        */

        let mut info_msg: Message = ChannelId(LEADERBOARDS_CHANNEL)
            .message(&ctx.http, 863385529831260221)
            .await
            .unwrap();
        info_msg.edit(&ctx.http, |message| {
            message.content("\n
                **Welcome to the stats leaderboards channel!**\n\n\
                In this channel you can see leaderboards for various Minecraft \
                statistics. Those are the stats for the current EstillaCraft \
                season.\n\n\
                The leaderboards are updated every 5 minutes, but remember that your \
                stats are only updated when you log off the server!\n\n\
                The displayed stats can be changed and if you want a certain \
                stat to be displayed, you can ask Elzapat to update it.\n\n\
            ")
        })
        .await
        .unwrap();
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

    // let _future = tokio::task::spawn(schedule_leaderboards(http));

    if let Err(e) = client.start().await {
        println!("Client error: {}", e);
    }
}
