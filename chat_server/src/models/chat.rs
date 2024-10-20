use serde::{Deserialize, Serialize};

use super::Chat;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateChat {
    pub name: Option<String>,
    pub members: Vec<i64>,
}

impl Chat {
    // pub async fn create(input:CreateChat,ws_id:u64, pool: &sqlx::PgPool) -> Result<Self, AppError> {
    //     let chat = sqlx::query_as(
    //         r#"
    //         INSERT INTO chats(ws_id,name,type,members)
    //         VALUES($1,$2,$3,$4)
    //         RETURNING id,ws_id,name,type,members,created_at
    //         "#,
    //     )
    //     .bind(ws_id)
    //     .bind(input.name)
    //     .bind(input.)
    //     .bind(&members)
    //     .fetch_one(pool)
    //     .await?;
    //     Ok(chat)
    // }

    // pub async fn find_by_id(id: i64, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
    //     let chat = sqlx::query_as(
    //         r#"
    //         SELECT id,ws_id,name,type,members,created_at
    //         FROM chats
    //         WHERE id=$1
    //         "#,
    //     )
    //     .bind(id)
    //     .fetch_optional(pool)
    //     .await?;
    //     Ok(chat)
    // }

    // pub async fn find_by_ws_id(ws_id: i64, pool: &sqlx::PgPool) -> Result<Vec<Self>, AppError> {
    //     let chats = sqlx::query_as(
    //         r#"
    //         SELECT id,ws_id,name,type,members,created_at
    //         FROM chats
    //         WHERE ws_id=$1
    //         "#,
    //     )
    //     .bind(ws_id)
    //     .fetch_all(pool)
    //     .await?;
    //     Ok(chats)
    // }

    // pub async fn find_by_member_id(member_id: i64, pool: &sqlx::PgPool) -> Result<Vec<Self>, AppError> {
    //     let chats = sqlx::query_as(
    //         r#"
    //         SELECT id,ws_id,name,type,members,created_at
    //         FROM chats
    //         WHERE $1 = ANY(members)
    //         "#,
    //     )
    //     .bind(member_id)
    //     .fetch_all(pool)
    //     .await?;
    //     Ok(chats)
    // }

    // pub async fn add_member(&self, member_id: i64, pool: &sqlx::PgPool) -> Result<Self, AppError> {
    //     let chat = sqlx::query_as(
    //         r#"
    //         UPDATE chats
    //         SET members = array_append(members,$1)
}
