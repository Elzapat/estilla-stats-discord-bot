pub mod bot_error;
mod application_commands;
mod stat;

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
            ApplicationCommandInteractionDataOption,
            // ApplicationCommandInteractionDataOptionValue,
        },
        event::ResumedEvent,
        channel::Message,
    },
    prelude::*,
};

use crate::application_commands::create_application_commands;
use stat::get_stat;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(InteractionData::ApplicationCommand(ref command)) = interaction.data {
            let content = match command.name.as_str() {
                "stat" => {
                    let (player, stat_type, stat_value);
                    for option in command.options {
                        match option.name.as_str() {
                            "player" => player = option.value.unwrap().to_string(),
                            "stat-type" => stat_type = option.value.unwrap().to_string(),
                            "stat-value" => stat_value = option.value.unwrap().to_string(),
                            _ => {},
                        }
                    }

                    let stat_result = get_stat(player, stat_type, stat_value).await;

                    if let Err(e) = interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    match stat_result {
                                        Err(e) => message.content(match e {

                                        }),
                                        Ok(stat) => message.embed(|e| {
                                            e.author("EstillaStats")
                                        })
                                    }
                                })
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", e)
                    }

                    if let Err(e) = get_stat(player, stat_type, stat_value).await {
                        match e {

                        }
                    }
                },
                _ => "not implemented :(".to_string(),
            };

            if let Err(e) = interaction 
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", e);
            }
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
