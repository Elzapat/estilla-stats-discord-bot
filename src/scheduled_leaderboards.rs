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
};

#[derive(Debug)]
struct Leaderboard<'a> {
    stat_type: &'a str,
    stat_name: &'a str,
    message_id: u64,
}

const INTERVAL: std::time::Duration = std::time::Duration::from_secs(60 * 10);
const CHANNEL_ID: u64 = 863383101841735701;
const LEADERBOARDS: [Leaderboard; 11] = [
    Leaderboard { stat_type: "custom", stat_name: "play one minute", message_id: 863385381861064734 },
    Leaderboard { stat_type: "mined", stat_name: "diamond ore", message_id: 863385396758446114 },
    Leaderboard { stat_type: "broken", stat_name: "wooden pickaxe", message_id: 863385400893898752 },
    Leaderboard { stat_type: "custom", stat_name: "jump", message_id: 863385405213376524 },
    Leaderboard { stat_type: "custom", stat_name: "deaths", message_id: 863385408715882517 },
    Leaderboard { stat_type: "custom", stat_name: "mob kills", message_id: 863385414315671562 },
    Leaderboard { stat_type: "custom", stat_name: "aviate one cm", message_id: 863385502768824331 },
    Leaderboard { stat_type: "custom", stat_name: "walk one cm", message_id: 863385507185819689 },
    Leaderboard { stat_type: "custom", stat_name: "sprint one cm", message_id: 863385510482935829 },
    Leaderboard { stat_type: "custom", stat_name: "damage taken", message_id: 863385514055565342 },
    Leaderboard { stat_type: "custom", stat_name: "damage dealt", message_id: 863385517637632041 },
];

pub async fn schedule_leaderboards(http: impl AsRef<Http> + CacheHttp + 'static) -> BotResult<()> {
    // Update leaderboards every ten minutes
    let mut interval_timer = tokio::time::interval(INTERVAL);

    loop {
        interval_timer.tick().await;
    println!("test");
        if let Err(e) = update_leaderboards(&http).await {
            println!("Error updating scoreboards: {:?}", e);
        }
    }
}

async fn update_leaderboards(http: impl AsRef<Http> + CacheHttp) -> BotResult<()> {
    for leaderboard in LEADERBOARDS.iter() {
        let http = &http;

        let mut msg: Message = ChannelId(CHANNEL_ID)
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
