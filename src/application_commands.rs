use serenity::{
    builder::CreateApplicationCommands,
    model::{
        interactions::{
            ApplicationCommandOptionType,
        },
    },
};

pub fn create_application_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        .create_application_command(|command|{
            command
                .name("stat")
                .description("Get one stat for a specific player")
                .create_option(|option| {
                    option
                        .name("player")
                        .description("Minecraft username of the plyer you want to see the specified stat for")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                })
                .create_option(|option| {
                    option
                        .name("stat-type")
                        .description("The type of the stat you want")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                        .add_string_choice("killed", "killed")
                        .add_string_choice("mined", "mined")
                        .add_string_choice("broken", "broken")
                        .add_string_choice("dropped", "dropped")
                        .add_string_choice("crafted", "crafted")
                        .add_string_choice("used", "used")
                        .add_string_choice("killed by", "killed by")
                        .add_string_choice("custom", "custom")
                })
                .create_option(|option| {
                    option
                        .name("stat-value")
                        .description("The value of the stat you want (Any block, item or mob. Can be incompatible with the type you chose)")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                })
        })
        .create_application_command(|command| {
            command
                .name("leaderboard")
                .description("Get the leaderboard for a specific stat")
                .create_option(|option| {
                    option
                        .name("stat-type")
                        .description("The type of the stat you want")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                        .add_string_choice("killed", "killed")
                        .add_string_choice("mined", "mined")
                        .add_string_choice("broken", "broken")
                        .add_string_choice("dropped", "dropped")
                        .add_string_choice("crafted", "crafted")
                        .add_string_choice("used", "used")
                        .add_string_choice("killed by", "killed by")
                        .add_string_choice("custom", "custom")
                })
                .create_option(|option| {
                    option
                        .name("stat-value")
                        .description("The value of the stat you want (Any block, item or mob. Can be incompatible with the type you chose)")
                        .required(true)
                        .kind(ApplicationCommandOptionType::String)
                })
                .create_option(|option| {
                    option
                        .name("limit")
                        .description("Limit the number of players on the leaderboard (default: 10, max: 25)")
                        .required(false)
                        .kind(ApplicationCommandOptionType::Integer)
                }) 
        })
}
