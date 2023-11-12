use crate::{db, error::ServerError, models, telegram_api, utils::escape_message};
use bigdecimal::{BigDecimal, Zero};
use reqwest;
use std::collections::HashMap;

pub async fn send_total_stats(
    pool: &db::Pool,
    chat: &models::chats::Chat,
) -> Result<(), ServerError> {
    let user_daily_stats = db::total_stats::get_total_stats(&pool, &chat).await?;

    let mut message = String::new();
    message.push_str("*TOTAL STATS*\n");

    for (idx, user) in user_daily_stats.iter().enumerate() {
        message.push_str(&format!(
            "{}. [{}](tg://user?{}) {}\n",
            idx + 1,
            user.first_name,
            user.user_id,
            user.message_count.as_ref().unwrap_or(&BigDecimal::zero())
        ));
    }

    message.push_str(&format!(
        "*TOTAL: {}*",
        user_daily_stats
            .iter()
            .filter_map(|n| {
                match n.message_count.as_ref() {
                    Some(message_count) => Some(message_count),
                    None => None,
                }
            })
            .sum::<BigDecimal>()
    ));

    let client = reqwest::Client::new();
    let mut map = HashMap::<&str, String>::new();
    map.insert("chat_id", chat.chat_id.to_string());
    map.insert("text", escape_message(&message));
    map.insert("parse_mode", "MarkdownV2".to_string());
    let _response = client
        .post(format!("{}/sendMessage", telegram_api::get_api_url()))
        .json(&map)
        .send()
        .await
        .map_err(|_| crate::error::ServerError::Unknown)?;

    Ok(())
}
