use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;

use super::{Chat, ChatType, ChatUser};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChat {
    pub name: Option<String>,
    pub members: Vec<i64>,
    pub public: bool,
}

impl Chat {
    pub async fn create(input: CreateChat, ws_id: u64, pool: &PgPool) -> Result<Self, AppError> {
        //对话成员必须大于2人
        let chat_type = verify_chat_type(&input, pool).await?;

        let chat = sqlx::query_as(
            r#"
            INSERT INTO chats(ws_id,name,type,members)
            VALUES($1,$2,$3,$4)
            RETURNING id,ws_id,name,type,members,created_at
            "#,
        )
        .bind(ws_id as i64)
        .bind(input.name)
        .bind(chat_type)
        .bind(input.members)
        .fetch_one(pool)
        .await?;
        Ok(chat)
    }

    pub async fn fetch_all(ws_id: u64, pool: &PgPool) -> Result<Vec<Self>, AppError> {
        let chats = sqlx::query_as(
            r#"
            SELECT id,ws_id,name,type,members,created_at
            FROM chats
            WHERE ws_id=$1 order by created_at desc
            "#,
        )
        .bind(ws_id as i64)
        .fetch_all(pool)
        .await?;
        Ok(chats)
    }

    pub async fn get_by_id(id: i64, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let chat = sqlx::query_as(
            r#"
            SELECT id,ws_id,name,type,members,created_at
            FROM chats
            WHERE id=$1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(chat)
    }
    pub async fn update_chat(input: CreateChat, id: u64, pool: &PgPool) -> Result<Self, AppError> {
        let chat_type = verify_chat_type(&input, pool).await?;

        let chat = sqlx::query_as(
            r#"
            UPDATE chats
            SET name=$1,type=$2,members=$3
            WHERE id=$4
            RETURNING id,ws_id,name,type,members,created_at
            "#,
        )
        .bind(input.name)
        .bind(chat_type)
        .bind(input.members)
        .bind(id as i64)
        .fetch_one(pool)
        .await?;
        Ok(chat)
    }
    pub async fn delete_chat(id: u64, pool: &PgPool) -> Result<Self, AppError> {
        let chat = sqlx::query_as(
            r#"
            DELETE FROM chats
            WHERE id=$1
            RETURNING id,ws_id,name,type,members,created_at
            "#,
        )
        .bind(id as i64)
        .fetch_one(pool)
        .await?;
        Ok(chat)
    }
}

pub async fn verify_chat_type(input: &CreateChat, pool: &PgPool) -> Result<ChatType, AppError> {
    //对话成员必须大于2人
    let len = input.members.len();
    if len < 2 {
        return Err(AppError::CreateChatError(
            "Chat must have at least 2 members".to_string(),
        ));
    }

    if len > 8 && input.name.is_none() {
        return Err(AppError::CreateChatError(
            "Group chat with more than 8 members must have a name".to_string(),
        ));
    }

    //verify if all members exist
    let users = ChatUser::fetch_by_ids(&input.members, pool).await?;
    if users.len() != len {
        return Err(AppError::CreateChatError(
            "Some members do not exist".to_string(),
        ));
    }

    let chat_type = match (&input.name, len) {
        (None, 2) => ChatType::Single,
        (None, _) => ChatType::Group,
        (Some(_), _) => {
            if input.public {
                ChatType::PublicChannel
            } else {
                ChatType::PrivateChannel
            }
        }
    };
    Ok(chat_type)
}

#[cfg(test)]
impl CreateChat {
    pub fn new(name: &str, members: &[i64], public: bool) -> Self {
        let name = if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        };
        Self {
            name,
            members: members.to_vec(),
            public,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::get_test_pool;

    #[tokio::test]
    async fn test_create_chat() {
        let (_tdb, pool) = get_test_pool(None).await;
        let input = CreateChat::new("", &[1, 2], false);
        let ws_id = 1;
        let chat = Chat::create(input, ws_id, &pool)
            .await
            .expect("create chat failed");
        assert_eq!(chat.members.len(), 2);
        assert_eq!(chat.ws_id, ws_id as i64);
        assert_eq!(chat.r#type, ChatType::Single);
    }

    #[tokio::test]
    async fn test_create_group_chat() {
        let (_tdb, pool) = get_test_pool(None).await;
        let ws_id = 1;
        let members = &[1, 2, 3];
        let input = CreateChat::new("", members, false);
        let chat = Chat::create(input, ws_id, &pool).await.unwrap();
        assert_eq!(chat.members.len(), 3);
        assert_eq!(chat.ws_id, ws_id as i64);
        assert_eq!(chat.r#type, ChatType::Group);
    }

    #[tokio::test]
    async fn test_create_public_chat_with_name() {
        let (_tdb, pool) = get_test_pool(None).await;
        let ws_id = 1;
        let members = &[1, 2, 3];
        let input = CreateChat::new("public chat", members, true);
        let chat = Chat::create(input, ws_id, &pool).await.unwrap();
        assert_eq!(chat.members.len(), 3);
        assert_eq!(chat.ws_id, ws_id as i64);
        assert_eq!(chat.r#type, ChatType::PublicChannel);
    }
}
