use axum::response::IntoResponse;

pub(crate) async fn list_chat_handler() -> impl IntoResponse {
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
