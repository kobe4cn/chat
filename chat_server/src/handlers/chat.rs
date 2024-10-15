use axum::{response::IntoResponse, Extension};
use tracing::info;

use crate::User;

pub(crate) async fn list_chat_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    info!("user: {:?}", user);
    "list"
}

pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    "create"
}

pub(crate) async fn update_chat_handler() -> impl IntoResponse {
    "update"
}

pub(crate) async fn send_message_handler() -> impl IntoResponse {
    "send"
}
