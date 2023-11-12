use super::Pool;
use crate::error::ServerError;
use crate::models;
use crate::telegram_api;

pub async fn get_or_create_user(
    pool: &Pool,
    telegram_user: &telegram_api::User,
) -> Result<models::users::User, ServerError> {
    let user = sqlx::query_as!(
        models::users::User,
        r#"SELECT * FROM users WHERE user_id = $1"#,
        telegram_user.id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(u) = user {
        Ok(u)
    } else {
        let user_id = telegram_user.id;
        let first_name = &telegram_user.first_name;
        let last_name = telegram_user.last_name.clone().unwrap_or(String::new());
        let username = telegram_user.username.clone().unwrap_or(String::new());

        let user = sqlx::query_as!(
            models::users::User,
            r#"
                INSERT INTO users (user_id, first_name, last_name, username) 
                VALUES ($1, $2, $3, $4) 
                RETURNING *
            "#,
            user_id,
            first_name,
            last_name,
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
