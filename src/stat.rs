use crate::bot_error::{ BotResult, BotError };
use crate::utils::*;
use serde::Deserialize;

use num_format::{ Locale, ToFormattedString };

use serenity::{
    builder::CreateEmbed,
    model::interactions::ApplicationCommandInteractionDataOption,
};

pub struct StatCommandArgs {
    pub player: String,
    pub stat_type: String,
    pub stat_value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stat {
    pub success: bool,
    pub uuid: String,
    #[serde(skip_deserializing)]
    pub username: String,
    #[serde(rename = "stat")]
    pub value: u64,
}

pub async fn get_stat<S>(player: S, stat_type: S, stat_value: S) -> BotResult<Stat> 
    where S: Into<String>
{
    let mut uuid = get_uuid_from_username(player.into()).await?;

    // If the uuid is trimmed, untrim it (necessary for the stats API (yes it's not well made; I made it :(  ))
    if uuid.contains('-') {
        uuid = untrim_uuid(uuid);
    }

    // Transform the stat value and the stat type into coorect minecraft ids
    let stat_type = name_to_minecraft_id(stat_type.into());
    let stat_value = name_to_minecraft_id(stat_value.into());

    let stat = fetch_stat(uuid, stat_type, stat_value).await?;

    Ok(stat)
}

async fn fetch_stat<S>(uuid: S, stat_type: S, stat_value: S) -> BotResult<Stat>
where
    S: Into<String>
{
    let request = format!(
        "{}/stats?uuid={}&stat_type={}&stat_value={}",
        SERVER_ADDRESS, uuid.into(), stat_type.into(), stat_value.into()
    );

    let response = reqwest::get(request)
        .await?
        .json::<Vec<Stat>>()
        .await?;

    match response[0].success {
        true => Ok(response[0].clone()),
        false => Err(BotError::Error(response[0].uuid.clone()))
    }
}

pub fn parse_stat_args(
    args: &Vec<ApplicationCommandInteractionDataOption>
) -> StatCommandArgs {
    let mut args_iter = args.iter();
    let player = args_iter 
        .find(|&x| x.name.as_str() == "player")
        .unwrap()
        .value.as_ref()
        .unwrap()
        .to_string()
        .replace("\"", "");
    let stat_type = args_iter
        .find(|&x| x.name.as_str() == "stat-type")
        .unwrap()
        .value.as_ref()
        .unwrap()
        .to_string()
        .replace("\"", "");
    let stat_value = args_iter
        .find(|&x| x.name.as_str() == "stat-value")
        .unwrap()
        .value.as_ref()
        .unwrap()
        .to_string()
        .replace("\"", "");

    StatCommandArgs { player, stat_type, stat_value }
}

pub fn create_stat_embed<'a, S>(
    stat: u64,
    player: S,
    uuid: S,
    stat_type: S,
    stat_value: S,
    embed: &'a mut CreateEmbed
) -> &'a mut CreateEmbed
where
    S: Into<String>
{
    embed.title(&player.into());
    embed.thumbnail(format!("https://crafatar.com/avatars/{}", uuid.into()));

    let field_name = make_stat_title(&mut stat_type.into(), &mut stat_value.into());

    embed.field(field_name, stat.to_formatted_string(&Locale::en), false);

    embed.color((200, 255, 0));

    embed
}
