pub mod chats;
pub mod daily_stats;
pub mod total_stats;
pub mod users;

use chrono::Datelike;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::postgres::PgPool;

pub type Pool = PgPool;

pub async fn initialize_database(url: &str) -> Pool {
    let pool = PgPool::connect(url).await.expect("Failed to create pool.");
    crate::migrations::MIGRATOR
        .run(&pool)
        .await
        .expect("Migration failed!");
    pool
}

pub fn get_date_time() -> DateTime<Utc> {
    let now = Utc::now();
    DateTime::<Utc>::from_utc(
        NaiveDateTime::parse_from_str(
            &format!("{}/{:02}/{:02} 0:0", now.year(), now.month(), now.day()),
            "%Y/%m/%d %H:%M",
        )
        .unwrap(),
        Utc,
    )
}
