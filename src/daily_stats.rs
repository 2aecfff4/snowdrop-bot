use crate::{db, error::ServerError, models, telegram_api, utils::escape_message};
use reqwest;
use std::collections::HashMap;

pub async fn send_daily_stats(
    pool: &db::Pool,
    chat: &models::chats::Chat,
) -> Result<(), ServerError> {
    let daily_stats = db::daily_stats::get_daily_stats(&pool, &chat).await?;

    let mut msg = String::new();
    msg.push_str("*DAILY STATS*\n");
    for (idx, user) in daily_stats.iter().enumerate() {
        msg.push_str(&format!(
            "{}. [{}](tg://user?{}) {}\n",
            idx + 1,
            user.first_name,
            user.user_id,
            user.message_count
        ));
    }
    msg.push_str(&format!(
        "*TOTAL: {}*",
        daily_stats.iter().map(|x| x.message_count).sum::<i64>()
    ));

    let client = reqwest::Client::new();
    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("chat_id", chat.chat_id.to_string());
    map.insert("text", escape_message(&msg));
    map.insert("parse_mode", "MarkdownV2".to_string());
    let _response = client
        .post(format!("{}/sendMessage", telegram_api::get_api_url()))
        .json(&map)
        .send()
        .await
        .map_err(|_| crate::error::ServerError::Unknown)?;

    Ok(())
}
