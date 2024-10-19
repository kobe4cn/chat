mod config;
mod error;
mod handlers;
use anyhow::Context;
use handlers::*;
use middlewares::{set_layer, verify_token};
mod middlewares;
mod models;
mod utils;
use core::fmt;
pub use error::{AppError, ErrorOutput};
pub use models::{ChatUser, User, WorkSpace};

use std::{ops::Deref, sync::Arc};

use axum::{
    middleware::from_fn_with_state,
    routing::{get, patch, post},
    Router,
};
pub use config::AppConfig;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}
#[allow(unused)]

pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: utils::DecodingKey,
    pub(crate) ek: utils::EncodingKey,
    pub(crate) pool: sqlx::PgPool,
}
impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}

pub async fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state = AppState::try_new(config).await?;
    let api = Router::new()
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id",
            patch(update_chat_handler).post(send_message_handler),
        )
        .route("/chat/:id/messages", get(list_messages_handler))
        .route("/users", get(list_chat_users_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler));

    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state);
    Ok(set_layer(app))
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        let dk = utils::DecodingKey::load(&config.auth.pk).context("load dk failed")?;
        let ek = utils::EncodingKey::load(&config.auth.sk).context("load ek failed")?;
        let pool = sqlx::PgPool::connect(&config.server.db_url)
            .await
            .context("connect to db failed")?;

        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                dk,
                ek,
                pool,
            }),
        })
    }
}
