use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{AppError, AppState};

use super::{ChatFile, Message};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMessage {
    pub files: Vec<String>,
    pub content: String,
}

impl AppState {
    #[allow(unused)]
    pub async fn create_message(
        &self,
        input: CreateMessage,
        chat_id: u64,
        user_id: u64,
    ) -> Result<Message, AppError> {
        if input.content.is_empty() && input.files.is_empty() {
            return Err(AppError::MessageCreateError(
                "content or files is required".to_string(),
            ));
        }
        for s in &input.files {
            let base_dir = &self.config.server.base_dir;
            let file = ChatFile::from_str(s)?;
            if !file.path(base_dir).exists() {
                return Err(AppError::MessageCreateError("file not exists".to_string()));
            }
        }
        let pool = &self.pool;
        let message = sqlx::query_as(
            r#"
            INSERT INTO messages(chat_id,sender_id,content,files)
            VALUES($1,$2,$3,$4)
            RETURNING id,chat_id,sender_id,content,files,created_at
            "#,
        )
        .bind(chat_id as i64)
        .bind(user_id as i64)
        .bind(input.content)
        .bind(input.files)
        .fetch_one(pool)
        .await?;
        Ok(message)
    }
}
