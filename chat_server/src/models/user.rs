use std::mem;

use crate::{AppError, User};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

impl User {
    pub async fn find_by_email(email: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
        let user =
            sqlx::query_as(r#"SELECT id,fullname,email,created_at FROM users WHERE email=$1"#)
                .bind(email)
                .fetch_optional(pool)
                .await?;
        // match user {
        //     Some(user) => Ok(Some(user)),
        //     None => Ok(None),
        // }
        Ok(user)
    }
    pub async fn create(
        email: &str,
        fullname: &str,
        password: &str,
        pool: &sqlx::PgPool,
    ) -> Result<Self, AppError> {
        let password_hash = hash_password(password)?;
        let user = sqlx::query_as(
            r#"INSERT INTO users (fullname,email,password_hash) VALUES ($1,$2,$3) RETURNING id,fullname,email,created_at"#,
        )
        .bind(fullname)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }
    ///verify email and password
    pub async fn verify(
        email: &str,
        password: &str,
        pool: &sqlx::PgPool,
    ) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            r#"SELECT id,fullname,email,password_hash,created_at FROM users WHERE email=$1"#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;
        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                let is_valid = verify_password(password, &password_hash.unwrap_or_default())?;
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
        let user = User::create(email, "kevin yang", "test123456", &pool).await?;
        assert_eq!(user.email, email);
        assert_eq!(user.fullname, "kevin yang");

        let user = User::find_by_email(email, &pool).await?;
        assert!(user.is_some());
        assert_eq!(user.unwrap().email, email);

        let user = User::verify(email, "test123456", &pool).await?;
        assert!(user.is_some());
        assert!(user.unwrap().email == email);
        // do something with the pool

        // when tdb gets dropped, the database will be dropped
        Ok(())
    }
}
