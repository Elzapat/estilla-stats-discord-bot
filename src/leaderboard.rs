use crate::{
    stat::Stat,
    bot_error::BotResult,
    utils::*
};

use serenity::{
    builder::CreateEmbed,
    model::interactions::ApplicationCommandInteractionDataOption,
};

use num_format::{ Locale, ToFormattedString };

pub struct LeaderboardCommandArgs {
    pub stat_type: String,
    pub stat_name: String,
    pub limit: Option<i64>,
}


pub async fn get_leaderboard<S>(stat_type: S, stat_name: S, limit: Option<i64>) -> BotResult<Vec<Stat>>
where
    S: Into<String> + Clone
{
    let limit = match limit {
        None => 10,
        Some(l) => match l {
            0 => 1,
            1..=25 => l,
            _ => 25,
        }
    };

    let stat_type = name_to_minecraft_id(stat_type.into());
    let stat_name = name_to_minecraft_id(stat_name.into());

    let request = format!(
        "{}/stats?uuid=all&stat_type={}&stat_name={}",
        SERVER_ADDRESS, stat_type, stat_name
    );

    let mut stats = reqwest::get(request)
        .await?
        .json::<Vec<Stat>>()
        .await?;

    stats.sort_by(|a, b| b.value.cmp(&a.value));

    if stats.len() > limit as usize {
        stats.drain((limit as usize)..);
    }
    stats.drain_filter(|s| !s.success);

    let mut uuids = vec![];
    for s in stats.iter() {
        uuids.push(s.uuid.clone());
    }

    let names = get_usernames_from_uuids(uuids).await?;

    for (i, s) in stats.iter_mut().enumerate() {
        s.username = names[i].clone();
    }

    Ok(stats)
}

pub fn parse_leaderboard_args(
    args: &Vec<ApplicationCommandInteractionDataOption>
) -> LeaderboardCommandArgs {
    let mut args_iter = args.iter();
    let stat_type = args_iter
        .find(|&x| x.name.as_str() == "stat-type")
        .unwrap()
        .value.as_ref()
        .unwrap()
        .to_string()
        .replace("\"", "");
    let stat_name = args_iter
        .find(|&x| x.name.as_str() == "stat-name")
        .unwrap()
        .value.as_ref()
        .unwrap()
        .to_string()
        .replace("\"", "");
    let limit = args_iter
        .find(|&x| x.name.as_str() == "limit")
        .and_then(|data| data.value.as_ref())
        .and_then(|name| name.as_i64());

    LeaderboardCommandArgs { stat_type, stat_name, limit }
}

pub fn create_leaderboard_embed<'a, S>(
    leaderboard: Vec<Stat>,
    stat_type: S,
    stat_name: S,
    embed: &'a mut CreateEmbed
) -> &'a mut CreateEmbed
where
    S: Into<String> + Copy
{
    let stat_title = make_stat_title(&mut stat_type.into(), &mut stat_name.clone().into());
    embed.title(stat_title.clone());

    embed.color((200, 255, 0));

    let mut ranks = (1..leaderboard.len() + 1).map(|i| {
        match i {
            // 1 => "<:gold_ingot:863081302076424223>".to_string(),
            // 2 => "<:iron_ingot:863081302005514260>".to_string(),
            // 3 => "<:copper_ingot:863081302079963136>".to_string(),
            1..=9 => format!("{}  ", i),
            _ => format!("{} ", i),
        }
    }).collect::<Vec<String>>();
    if ranks.is_empty() {
        ranks.push("‎".to_string());
    }

    let mut names = leaderboard.iter().map(|s| {
        s.username.clone()
    }).collect::<Vec<String>>();
    if names.is_empty() {
        names.push("‎".to_string());
    }

    let mut stats = leaderboard
        .iter()
        .map(|s| if stat_name.into() == "play time" {
            minecraft_ticks_to_formatted_time(s.value)
        } else {
            s.value.to_formatted_string(&Locale::en)
        })
        .collect::<Vec<String>>();
    if stats.is_empty() {
        stats.push("‎".to_string());
    }

    let longest = names.iter().fold(0, |acc, name|
        if name.len() > acc { name.len() } else { acc }
    );

    let mut field_value = String::new();
    for i in 0..names.len() {
        field_value = format!("{}`{} {:<width$}  {}`\n", field_value, ranks[i], names[i], stats[i], width = longest);
    }

    embed.field("field", field_value, false);

    /*
    let mut fields = vec![];
    let mut titles = vec![];
    for (rank, name) in ranks.iter().zip(names.iter()) {
        titles.push(format!("{}{}", rank, name));
    }
    for (title, stat) in titles.iter().zip(stats.iter()) {
        fields.push((title, stat, false));
    }
    embed.fields(fields);
    */

    /*
    embed.fields(vec![
        ("Rank", ranks.join("\n"), true),
        ("Username", names.join("\n"), true),
        (stat_title.as_str(), stats.join("\n"), true)
    ]);
    */

    embed.footer(|f| f.text("This is not meant to be viewed on mobile"));

    embed
}
