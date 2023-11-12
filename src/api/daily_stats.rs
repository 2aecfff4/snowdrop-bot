use crate::{daily_stats::send_daily_stats, db};
use actix_web::{post, web, HttpResponse, Result};

//! https://core.telegram.org/bots/api#sendmessage

#[post("/daily_stats")]
async fn daily_stats(pool: web::Data<db::Pool>) -> Result<HttpResponse> {
    let chats = db::chats::get_chats(&pool).await?;

    for chat in chats.iter() {
        if chat.is_validated {
            send_daily_stats(&pool, chat).await?;
        }
    }

    Ok(HttpResponse::Ok().finish())
}
