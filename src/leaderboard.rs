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
    pub stat_value: String,
    pub limit: Option<i64>,
}


pub async fn get_leaderboard<S>(stat_type: S, stat_value: S, limit: Option<i64>) -> BotResult<Vec<Stat>>
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
    let stat_value = args_iter
        .find(|&x| x.name.as_str() == "stat-value")
        .unwrap()
        .value.as_ref()
        .unwrap()
        .to_string()
        .replace("\"", "");
    let limit = args_iter
        .find(|&x| x.name.as_str() == "limit")
        .and_then(|data| data.value.as_ref())
        .and_then(|value| value.as_i64());

    LeaderboardCommandArgs { stat_type, stat_value, limit }
}

pub fn create_leaderboard_embed<'a, S>(
    leaderboard: Vec<Stat>,
    stat_type: S,
    stat_value: S,
    embed: &'a mut CreateEmbed
) -> &'a mut CreateEmbed
where
    S: Into<String>
{
    let stat_title = make_stat_title(&mut stat_type.into(), &mut stat_value.into());
    embed.title(stat_title.clone());

    embed.color((200, 255, 0));

    let ranks = (1..leaderboard.len() + 1).map(|i| {
        match i {
            1 => "<:gold_ingot:863081302076424223>".to_string(),
            2 => "<:iron_ingot:863081302005514260>".to_string(),
            3 => "<:copper_ingot:863081302079963136>".to_string(),
            _ => i.to_string(),
        }
    }).collect::<Vec<String>>();
    // let ranks_len: usize = 2;
    // ranks = ranks.iter().map(|rank| format!("{:⠀<1$}", rank, ranks_len)).collect();

    let names = leaderboard.iter().map(|s| {
        s.username.clone()
    }).collect::<Vec<String>>();
    // let names_len = longest_length_in_string_vec(&names);
    // names = names.iter().map(|name| format!("{:⠀<1$}", name, names_len)).collect();

    let stats = leaderboard
        .iter()
        .map(|s| s.value.to_formatted_string(&Locale::en))
        .collect::<Vec<String>>();
    // let stats_len = longest_length_in_string_vec(&names);
    // stats = stats.iter().map(|stat| format!("{:⠀<1$}", stat, stats_len)).collect();

    // embed.field(
    //     "test",
    //     izip!(ranks, names, stats)
    //         .map(|(rank, name, stat)| { 
    //             format!("{}{}{}", rank, name, stat)
    //         })
    //         .collect::<Vec<String>>()
    //         .join("\n"),
    //     true
    // );

    embed.fields(vec![
        ("Rank", ranks.join("\n"), true),
        ("Username", names.join("\n"), true),
        (stat_title.as_str(), stats.join("\n"), true)
    ]);

    embed.footer(|f| f.text("This is not meant to be viewed on mobile"));

    embed
}
