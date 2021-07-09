use crate::stat::Stat;
use crate::bot_error::{ BotResult };
use crate::utils::*;

pub async fn get_leaderboard<S>(stat_type: S, stat_value: S, limit: Option<u8>) -> BotResult<Vec<Stat>>
    where S: Into<String>
{
    let limit = match limit {
        None => 10,
        Some(l) => match l {
            0 => 1,
            25..=255 => 25,     
            _ => l,
        }
    };

    let stat_type = name_to_minecraft_id(stat_type.into());
    let stat_value = name_to_minecraft_id(stat_value.into());

    let request = format!(
        "{}/stats?uuid=all&stat_type={}&stat_value={}",
        SERVER_ADDRESS, stat_type, stat_value
    );

    let mut stats = reqwest::get(request)
        .await?
        .json::<Vec<Stat>>()
        .await?;

    stats.sort_by(|a, b| b.value.cmp(&a.value));

    stats.drain((limit as usize)..);

    Ok(stats)
}
