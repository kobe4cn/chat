mod chat;
mod file;
mod message;
mod user;
mod workspace;
pub use chat::CreateChat;
use chrono::{DateTime, Utc};
pub use message::{CreateMessage, ListMessages};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
pub use user::{CreateUser, SigninUser};

#[derive(Debug, Clone, FromRow, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub ws_id: i64,
    pub fullname: String,
    pub email: String,
    #[serde(skip)]
    #[sqlx(default)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Deserialize, Serialize, PartialEq)]
pub struct ChatUser {
    pub id: i64,
    pub fullname: String,
    pub email: String,
}

#[derive(Debug, Clone, FromRow, Deserialize, Serialize, PartialEq)]
pub struct WorkSpace {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Chat {
    pub id: i64,
    pub ws_id: i64,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub members: Vec<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, sqlx::Type, PartialOrd)]
#[sqlx(type_name = "chat_type", rename_all = "snake_case")]
pub enum ChatType {
    Single,
    Group,
    PrivateChannel,
    PublicChannel,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatFile {
    pub ws_id: i64,
    pub hash: String,
    pub ext: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: String,
    pub files: Vec<String>,
    pub created_at: DateTime<Utc>,
}
