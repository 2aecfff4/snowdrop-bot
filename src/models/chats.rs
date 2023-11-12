use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Chat {
    pub id: i64,
    pub chat_id: i64,
    pub title: String,
    pub is_validated: bool,
}
