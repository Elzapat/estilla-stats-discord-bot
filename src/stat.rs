use crate::bot_error::{ BotResult, BotError };

use serde::Deserialize;

struct Stat {
    player_uuid: String,
    stat_value: String,
    stat_type: String,
    stat: u64,
}

#[derive(Deserialize)]
struct MinecraftPlayer {
    name: String,
    id: String,
}

pub async fn get_stat(player: String, mut stat_type: String, mut stat_value: String) -> BotResult<Stat> {
    let mut uuid = get_uuid_from_username(player).await?;

    // If the uuid is trimmed, untrim it (necessary for the stats API (yes it's not well made; I made it :(  ))
    if uuid.contains('-') {
        uuid = untrim_uuid(uuid);
    }

    // Transform the stat value and the stat type into coorect minecraft ids
    stat_type = format!("minecraft:{}", stat_type.replace(" ", "_").to_lowercase());
    stat_value = format!("minecraft:{}", stat_value.replace(" ", "_").to_lowercase());

    let stat = fetch_stat(&uuid, &stat_value, &stat_type).await?;

    Ok(Stat { player_uuid: uuid, stat_value, stat_type, stat: stat })
}

fn untrim_uuid(uuid: String) -> String {
    format!("{}-{}-{}-{}-{}", &uuid[0..8], &uuid[8..12], &uuid[12..16], &uuid[16..20], &uuid[20..32])
}

async fn get_uuid_from_username(username: String) -> BotResult<String> {
    // Use the Mojang API to get the UUID of the player
    let request = format!("https://api.mojang.com/users/profiles/minecraft/{}", username);
    let response = reqwest::get(request).await?;

    // Response is successful but there is no match for the give username
    if response.status().as_u16() == 204 {
        return Err(BotError::Error("Error: This username doesn't exist".to_string()));
    }

    let player = response.json::<MinecraftPlayer>().await?;

    Ok(player.id)
}

async fn fetch_stat(uuid: &String, stat_type: &String, stat_value: &String) -> BotResult<u64> {
    const SERVER_ADDRESS: &str = "77.75.125.164";
    let request = format!(
        "{}/stats?uuid={}&stat_type={}&stat_value={}",
        SERVER_ADDRESS, uuid, stat_type, stat_value
    );

    #[derive(Deserialize)]
    struct Response {
        success: bool,
        uuid: String,
        stat: u64,
    }

    let response = reqwest::get(request)
        .await?
        .json::<Response>()
        .await?;

    match response.success {
        true => Ok(response.stat),
        false => Err(BotError::Error(response.uuid))
    }
}
