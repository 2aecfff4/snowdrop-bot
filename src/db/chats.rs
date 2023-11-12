use super::Pool;
use crate::error::ServerError;
use crate::models;
use crate::telegram_api;

pub async fn get_or_create_chat(
    pool: &Pool,
    telegram_chat: &telegram_api::Chat,
) -> Result<models::chats::Chat, ServerError> {
    let chat = sqlx::query_as!(
        models::chats::Chat,
        r#"SELECT * FROM chats WHERE chat_id = $1"#,
        telegram_chat.id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(c) = chat {
        Ok(c)
    } else {
        let title = telegram_chat.title.clone().unwrap_or(String::new());

        let chat = sqlx::query_as!(
            models::chats::Chat,
            r#"
                INSERT INTO chats (chat_id, title) 
                VALUES ($1, $2) 
                RETURNING *
            "#,
            telegram_chat.id,
            title
        )
        .fetch_one(pool)
        .await?;

        Ok(chat)
    }
}

pub async fn get_chats(pool: &Pool) -> Result<Vec<models::chats::Chat>, ServerError> {
    let chats = sqlx::query_as!(models::chats::Chat, r#"SELECT * FROM chats"#)
        .fetch_all(pool)
        .await?;

    Ok(chats)
}

pub async fn get_chat(
    pool: &Pool,
    telegram_chat: &telegram_api::Chat,
) -> Result<Option<models::chats::Chat>, ServerError> {
    let chat = sqlx::query_as!(
        models::chats::Chat,
        r#"SELECT * FROM chats WHERE chat_id = $1"#,
        telegram_chat.id
    )
    .fetch_optional(pool)
    .await?;

    Ok(chat)
}
