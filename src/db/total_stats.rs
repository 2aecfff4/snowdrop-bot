use super::Pool;
use crate::error::ServerError;
use crate::models;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserTotalStats {
    pub user_id: i64,
    pub first_name: String,
    // #TODO: Why does this need to be an option? Even with `COALESCE`.
    pub message_count: Option<BigDecimal>,
}

/// Stats are ordered descending.
pub async fn get_total_stats(
    pool: &Pool,
    chat: &models::chats::Chat,
) -> Result<Vec<UserTotalStats>, ServerError> {
    let chats = sqlx::query_as!(
        UserTotalStats,
        r#"
            SELECT daily_stats.user_id, users.first_name, (SUM(daily_stats.message_count) + old_stats.message_count) as message_count
            FROM daily_stats
            INNER JOIN users ON daily_stats.user_id = users.user_id
            INNER JOIN old_stats ON daily_stats.user_id = old_stats.user_id
            WHERE daily_stats.chat_id = $1
            GROUP BY daily_stats.user_id, users.first_name, old_stats.message_count
            ORDER BY message_count DESC 
        "#,
        chat.chat_id
    )
    .fetch_all(pool)
    .await?;

    Ok(chats)
}
