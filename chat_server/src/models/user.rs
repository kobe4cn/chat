use std::mem;

use crate::{AppError, User};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

use super::{ChatUser, WorkSpace};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUser {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub workspace: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SigninUser {
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn find_by_email(email: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as(
            r#"SELECT id,ws_id,fullname,email,created_at FROM users WHERE email=$1"#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    // TODO: transaction thinking for workspace create rollback
    pub async fn create(input: &CreateUser, pool: &sqlx::PgPool) -> Result<Self, AppError> {
        //check if workspaces
        let user = Self::find_by_email(&input.email, pool).await?;
        if user.is_some() {
            return Err(AppError::EmailAlreadyExists(input.email.clone()));
        }
        let ws = match WorkSpace::find_by_name(&input.workspace, pool).await? {
            Some(ws) => ws,
            None => WorkSpace::create(&input.workspace, 0, pool).await?,
        };

        let password_hash = hash_password(&input.password)?;
        let user:User = sqlx::query_as(
            r#"INSERT INTO users (ws_id,fullname,email,password_hash) VALUES ($1,$2,$3,$4) RETURNING id,ws_id,fullname,email,created_at"#,
        )
        .bind(ws.id)
        .bind(&input.fullname)
        .bind(&input.email)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        ws.update_owner(user.id as _, pool).await?;
        Ok(user)
    }

    pub async fn add_to_workspace(
        &self,
        workspace_id: u64,
        pool: &sqlx::PgPool,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as(
            r#"UPDATE users SET ws_id=$1 WHERE id=$2
        RETURNING id,ws_id,fullname,email,created_at
        "#,
        )
        .bind(workspace_id as i64)
        .bind(self.id)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }
    ///verify email and password
    pub async fn verify(input: &SigninUser, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            r#"SELECT id,ws_id,fullname,email,password_hash,created_at FROM users WHERE email=$1"#,
        )
        .bind(&input.email)
        .fetch_optional(pool)
        .await?;
        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                let is_valid =
                    verify_password(&input.password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

impl ChatUser {
    // pub async fn fetch_all()
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;
    let is_valid = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_valid)
}

#[cfg(test)]
impl CreateUser {
    pub fn new(fullname: &str, email: &str, password: &str) -> Self {
        Self {
            workspace: "default".to_string(),
            fullname: fullname.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

impl User {
    pub fn new(id: i64, fullname: &str, email: &str) -> Self {
        Self {
            id,
            ws_id: 0,
            fullname: fullname.to_string(),
            email: email.to_string(),
            password_hash: None,
            created_at: chrono::Utc::now(),
        }
    }
}
impl SigninUser {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}
#[cfg(test)]

mod tests {
    use super::*;

    use sqlx_db_tester::TestPg;

    #[tokio::test]

    async fn create_and_verify_user_should_work() -> Result<(), AppError> {
        let tdb = TestPg::new(
            "postgres://postgres:postgres@localhost:5432/chat".to_string(),
            std::path::Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let email = "kevin.yang.xgz@gamil.com";
        let input = CreateUser::new("kevin yang", email, "password123456");
        let user = User::create(&input, &pool).await?;
        assert_eq!(user.email, email);
        assert_eq!(user.fullname, "kevin yang");

        let user = User::find_by_email(email, &pool).await?;
        assert!(user.is_some());
        assert_eq!(user.unwrap().email, email);
        let signinuser = SigninUser::new(email, "password123456");
        let user = User::verify(&signinuser, &pool).await?;
        assert!(user.is_some());
        assert!(user.unwrap().email == email);
        // do something with the pool

        // when tdb gets dropped, the database will be dropped
        Ok(())
    }
}
