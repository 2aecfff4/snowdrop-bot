use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#user>
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

/// <https://core.telegram.org/bots/api#chat>
#[derive(Deserialize, Serialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub title: Option<String>,
}

/// <https://core.telegram.org/bots/api#message>
#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub date: i64,
    pub chat: Chat,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
}

/// <hhttps://core.telegram.org/bots/api#messageentity>
#[derive(Deserialize, Serialize, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub message_type: String,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
}

/// <https://core.telegram.org/bots/api#update>
#[derive(Deserialize, Serialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
}

pub fn get_api_url() -> &'static str {
    lazy_static! {
        static ref URL: String = format!(
            "https://api.telegram.org/bot{}",
            std::env::var("BOT_TOKEN").unwrap()
        );
    }

    &URL
}
