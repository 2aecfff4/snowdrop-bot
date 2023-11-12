use crate::{
    db,
    error::ServerError,
    telegram_api::{Message, Update},
};
use actix_web::{post, web, HttpResponse, Result};

#[post("/webhook")]
async fn webhook(pool: web::Data<db::Pool>, update: web::Json<Update>) -> Result<HttpResponse> {
    if update.message.is_none() || update.message.as_ref().unwrap().from.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }

    let message = update.message.as_ref().unwrap();
    let from = message.from.as_ref().unwrap();

    let chat = db::chats::get_or_create_chat(&pool, &message.chat).await?;
    if !chat.is_validated {
        // This is an error from our perspective (unauthorized access),
        // but return `Ok` to make telegram happy.
        Ok(HttpResponse::Ok().finish())
    } else {
        let user = db::users::get_or_create_user(&pool, &from).await?;
        db::daily_stats::update_or_insert_daily_stats(&pool, &chat, &user).await?;
        handle_bot_commands(&pool, message).await?;

        Ok(HttpResponse::Ok().finish())
    }
}

async fn handle_bot_commands(pool: &db::Pool, message: &Message) -> Result<(), ServerError> {
    if message.entities.is_some() && message.text.is_some() {
        let text = message.text.as_ref().unwrap();
        let entities = message.entities.as_ref().unwrap();
        for entity in entities.iter() {
            if entity.message_type == "bot_command" {
                let length = text.find("@").unwrap_or(0);
                let command: String = text
                    .chars()
                    .skip(entity.offset as usize)
                    .take(length)
                    .collect();

                match command.as_str() {
                    "/stats" => {
                        let chat = db::chats::get_chat(pool, &message.chat).await?;
                        if let Some(c) = chat {
                            crate::total_stats::send_total_stats(pool, &c).await?;
                        }
                    }
                    "/dailystats" => {
                        let chat = db::chats::get_chat(pool, &message.chat).await?;
                        if let Some(c) = chat {
                            crate::daily_stats::send_daily_stats(pool, &c).await?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
