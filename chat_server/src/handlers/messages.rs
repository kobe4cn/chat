use axum::response::IntoResponse;

pub(crate) async fn list_messages_handler() -> impl IntoResponse {
    "list messages"
}
