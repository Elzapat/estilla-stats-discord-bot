use crate::bot_error::{ BotResult, BotError };
use serde::Deserialize;
use futures::{ stream, StreamExt };

pub const SERVER_ADDRESS: &str = "http://estillacraft.net";
pub const LEADERBOARDS_CHANNEL: u64 = 863383101841735701;

pub fn make_ascii_titlecase(s: &mut str) -> String {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }

    s.to_string()
}

pub fn name_to_minecraft_id(name: String) -> String {
    format!("minecraft:{}", name.replace(" ", "_").to_lowercase())
}

pub fn make_stat_title(mut stat_type: &mut String, mut stat_name: &mut String) -> String {
    if stat_name.chars().last().unwrap() != 's' && stat_type != "custom" {
        stat_name.push_str("s");
    }

    match stat_type.as_str() {
        "custom" => make_ascii_titlecase(&mut stat_name),
        "killed by" => format!("{} {}", make_ascii_titlecase(&mut stat_type), stat_name),
        _ => format!("{} {}", make_ascii_titlecase(&mut stat_name), stat_type),
    }
}

pub fn untrim_uuid(uuid: String) -> String {
    format!("{}-{}-{}-{}-{}", &uuid[0..8], &uuid[8..12], &uuid[12..16], &uuid[16..20], &uuid[20..32])
}

pub async fn get_uuid_from_username<S>(username: S) -> BotResult<String>
    where S: Into<String>
{
    #[derive(Deserialize)]
    struct MinecraftPlayer {
        id: String,
    }

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

pub async fn get_username_from_uuid<S>(uuid: S) -> BotResult<String>
    where S: Into<String>
{
    #[derive(Deserialize)]
    struct PlayerName {
        name: String,
    }

    let request = format!("https://api.mojang.com/user/profiles/{}/names", uuid.into());
    println!("{}", request);

    let response = reqwest::get(request).await?;

    if response.status().as_u16() != 200 {
        return Err(BotError::Error("Invalid UUID".to_string()));
    }

    let name_history = response.json::<Vec<PlayerName>>().await?;

    Ok(name_history.last().unwrap().name.clone())
}

pub async fn get_usernames_from_uuids(uuids: Vec<String>) -> BotResult<Vec<String>> {
    const CONCURRENT_REQUESTS: usize = 10;

    #[derive(Debug, Deserialize)]
    struct PlayerName {
        name: String,
    }

    let client = reqwest::Client::new();

    let urls = uuids.iter().map(|uuid|
        format!("https://api.mojang.com/user/profiles/{}/names", uuid)
    ).collect::<Vec<String>>();

    let name_histories = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url).send().await?;
                resp.json::<Vec<PlayerName>>().await
            }
        })
        .buffered(CONCURRENT_REQUESTS);

    let names = name_histories
        .collect::<Vec<_>>()
        .await
        .iter()
        .map(|name_history| match name_history {
            Ok(name_history) => name_history.last().unwrap().name.clone()/*.replace("_", "\\_")*/,
            Err(e) => e.to_string(),
        })
        .collect();

    Ok(names)
}

pub fn longest_length_in_string_vec(source: &Vec<String>) -> usize {
    source.iter().fold(1, |acc, item| {
        if item.len() > acc {
            item.len()
        } else {
            acc
        }
    })
}

pub fn minecraft_ticks_to_formatted_time(ticks: u64) -> String {
    let mut seconds = ticks / 20;
    let mut minutes = seconds / 60;
    let mut hours = minutes / 60;
    let days = hours / 24;

    seconds = seconds % 60;
    minutes = minutes % 60;
    hours = hours % 24;

    let days_s = if days > 0 { format!("{}d ", days) } else { "".to_string() }; 
    let hours_s = if days > 0 || hours > 0 { format!("{:2}h ", hours) } else { "".to_string() };
    let minutes_s = if days > 0 || hours > 0 || minutes > 0 { format!("{:2}m ", minutes) } else { "".to_string() };
    let seconds_s = format!("{:2}s", seconds);

    format!("{}{}{}{}", days_s, hours_s, minutes_s, seconds_s)
}
