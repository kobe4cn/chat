use crate::AppError;

use crate::{ChatUser, WorkSpace};

impl WorkSpace {
    pub async fn create(name: &str, user_id: u64, pool: &sqlx::PgPool) -> Result<Self, AppError> {
        let workspace = sqlx::query_as(
            r#"
            INSERT INTO workspaces(name,owner_id)
            VALUES($1,$2)
            RETURNING id,name,owner_id,created_at
            "#,
        )
        .bind(name)
        .bind(user_id as i64)
        .fetch_one(pool)
        .await?;
        Ok(workspace)
    }

    pub async fn find_by_name(name: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
        let workspace = sqlx::query_as(
            r#"
            SELECT id,name,owner_id,created_at
            FROM workspaces
            WHERE name=$1
            "#,
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        Ok(workspace)
    }

    pub async fn find_by_id(id: u64, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
        let workspace = sqlx::query_as(
            r#"
            SELECT id,name,owner_id,created_at
            FROM workspaces
            WHERE id=$1
            "#,
        )
        .bind(id as i64)
        .fetch_optional(pool)
        .await?;
        Ok(workspace)
    }

    pub async fn update_owner(
        &self,
        new_owner_id: u64,
        pool: &sqlx::PgPool,
    ) -> Result<Self, AppError> {
        let workspace = sqlx::query_as(
            r#"
            UPDATE workspaces
            SET owner_id = $1
            WHERE id = $2 and (select ws_id from users where id=$1)=$2
            RETURNING id,name,owner_id,created_at
            "#,
        )
        .bind(new_owner_id as i64)
        .bind(self.id)
        .fetch_one(pool)
        .await?;
        Ok(workspace)
    }

    pub async fn fetch_all_chat_users(
        workspace_id: u64,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<ChatUser>, AppError> {
        let users = sqlx::query_as(
            r#"
            SELECT id,fullname,email
            FROM users
            WHERE ws_id = $1 order by id
            "#,
        )
        .bind(workspace_id as i64)
        .fetch_all(pool)
        .await?;
        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_pool;

    use super::*;

    use anyhow::Result;

    #[tokio::test]
    async fn test_create_workspace() -> Result<()> {
        let (_tdb, pool) = get_test_pool(None).await;
        let ws = WorkSpace::create("test", 0, &pool).await?;
        assert_eq!(ws.name, "test");
        Ok(())
    }
}
