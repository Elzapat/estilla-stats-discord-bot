use crate::bot_error::{ BotResult, BotError };
use crate::utils::*;
use serde::Deserialize;
// use serenity::model::interactions::ApplicationCommandInteractionDataOption;

#[derive(Debug, Clone, Deserialize)]
pub struct Stat {
    pub success: bool,
    pub uuid: String,
    #[serde(rename = "stat")]
    pub value: u64,
}

#[derive(Deserialize)]
struct MinecraftPlayer {
    name: String,
    id: String,
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

fn untrim_uuid(uuid: String) -> String {
    format!("{}-{}-{}-{}-{}", &uuid[0..8], &uuid[8..12], &uuid[12..16], &uuid[16..20], &uuid[20..32])
}

pub async fn get_uuid_from_username<S>(username: S) -> BotResult<String>
    where S: Into<String>
{
    // Use the Mojang API to get the UUID of the player
    let request = format!("https://api.mojang.com/users/profiles/minecraft/{}", username.into());
    let response = reqwest::get(request).await?;


    // Response is successful but there is no match for the given username
    if response.status().as_u16() == 204 {
        return Err(BotError::Error("Error: This username doesn't exist".to_string()));
    }

    let player = response.json::<MinecraftPlayer>().await?;

    Ok(player.id)
}

async fn fetch_stat<S>(uuid: S, stat_type: S, stat_value: S) -> BotResult<Stat>
    where S: Into<String>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn uuid_from_username() {
        assert_eq!(get_uuid_from_username("Elzapat".to_string()).await.unwrap(), "bb1784e458ee40749ae248684656aa59");
    }

    #[test]
    fn uuid_untrimming() {
        assert_eq!(untrim_uuid("bb1784e458ee40749ae248684656aa59".to_string()), "bb1784e4-58ee-4074-9ae2-48684656aa59")
    }
}
