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
            ApplicationCommand,
            ApplicationCommandOptionType,
            // ApplicationCommandInteractionData,
            // ApplicationCommandInteractionDataOptionValue,
        },
        event::ResumedEvent,
        channel::Message,
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(InteractionData::ApplicationCommand(ref command)) = interaction.data {
            let content = match command.name.as_str() {
                "ping" => "pong!".to_string(),
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

        let commands = ApplicationCommand::create_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("test").description("test2")
                        .create_option(|option| {
                            option.name("test-opt")
                                .description("test(desc)")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
        })
        .await;

        println!("I now have the following slash commands: {:?}", commands);

        let guild = GuildId(689495268185473059).to_partial_guild(&ctx.http).await.expect("aled guild id");
        

        let cmd = guild 
            .create_application_command(&ctx.http, |command| {
                command.name("wonderful_command").description("An amazing command")
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
