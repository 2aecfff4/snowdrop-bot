use super::get_date_time;
use super::Pool;
use crate::error::ServerError;
use crate::models;
use serde::{Deserialize, Serialize};

pub async fn update_or_insert_daily_stats(
    pool: &Pool,
    chat: &models::chats::Chat,
    user: &models::users::User,
) -> Result<(), ServerError> {
    let date = get_date_time();

    // #TODO: This logic is incorrect and is subject to race conditions.
    // It should be changed to `ON CONFLICT` / `INSERT ... ON DUPLICATE KEY UPDATE`.

    let daily_stats = sqlx::query_as!(
        models::daily_stats::DailyStats,
        r#"SELECT * FROM daily_stats WHERE user_id = $1 AND date = $2"#,
        user.user_id,
        date
    )
    .fetch_optional(pool)
    .await?;

    if let Some(stats) = daily_stats {
        let message_count = stats.message_count + 1;
        sqlx::query!(
            r#"
                UPDATE daily_stats 
                SET message_count = $1
                WHERE user_id = $2 AND date = $3
            "#,
            message_count,
            user.user_id,
            date
        )
        .execute(pool)
        .await?;
    } else {
        sqlx::query!(
            r#"
                INSERT INTO daily_stats (user_id, chat_id, message_count, date) 
                VALUES ($1, $2, $3, $4)
            "#,
            user.user_id,
            chat.chat_id,
            1,
            date
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDailyStats {
    pub user_id: i64,
    pub message_count: i64,
    pub first_name: String,
}

/// Stats are ordered descending.
pub async fn get_daily_stats(
    pool: &Pool,
    chat: &models::chats::Chat,
) -> Result<Vec<UserDailyStats>, ServerError> {
    let date = get_date_time();
    let chats = sqlx::query_as!(
        UserDailyStats,
        r#"
            SELECT daily_stats.user_id, daily_stats.message_count, users.first_name 
            FROM daily_stats
            INNER JOIN users ON daily_stats.user_id = users.user_id
            WHERE chat_id = $1 AND date = $2
            ORDER BY message_count DESC 
        "#,
        chat.chat_id,
        date
    )
    .fetch_all(pool)
    .await?;

    Ok(chats)
}
