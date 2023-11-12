use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DailyStats {
    pub id: i64,
    pub user_id: i64,
    pub chat_id: i64,
    pub message_count: i64,
    pub date: DateTime<Utc>,
}
