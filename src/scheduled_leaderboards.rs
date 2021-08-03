use serenity::{
    http::{
        CacheHttp,
        client::Http,
    },
    model::{
        channel::Message,
        id::ChannelId,
    },
};

use crate::{
    bot_error::BotResult,
    leaderboard::{
        get_leaderboard,
        create_leaderboard_embed,
    },
    utils::LEADERBOARDS_CHANNEL,
};

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Leaderboard<'a> {
    stat_type: &'a str,
    stat_name: &'a str,
    message_id: u64,
}

const INTERVAL: std::time::Duration = std::time::Duration::from_secs(60 * 5);

pub async fn schedule_leaderboards(http: impl AsRef<Http> + CacheHttp + 'static) -> BotResult<()> {
    let leaderboards = fs::read_to_string("leaderboards.ron")?;
    let leaderboards: Vec<Leaderboard> = ron::de::from_str(&leaderboards)?;

    // Update leaderboards every ten minutes
    let mut interval_timer = tokio::time::interval(INTERVAL);

    loop {
        interval_timer.tick().await;
        if let Err(e) = update_leaderboards(&http, &leaderboards).await {
            println!("Error updating scoreboards: {:?}", e);
        }
    }
}

async fn update_leaderboards(
    http: impl AsRef<Http> + CacheHttp,
    leaderboards: &Vec<Leaderboard<'_>>
) -> BotResult<()> {
    for leaderboard in leaderboards.iter() {
        let http = &http;

        let mut msg: Message = ChannelId(LEADERBOARDS_CHANNEL)
            .message(http, leaderboard.message_id)
            .await?;

        let stats = get_leaderboard(
            leaderboard.stat_type, leaderboard.stat_name, Some(10)
        ).await?;

        msg.edit(http, |message|
            message
                .content("")
                .embed(|e|
                    create_leaderboard_embed(
                        stats, leaderboard.stat_type,
                        leaderboard.stat_name, e
                    )
                )
        ).await?;
    }

    Ok(())
}
