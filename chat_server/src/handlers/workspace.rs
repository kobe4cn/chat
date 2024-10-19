use axum::{extract::State, response::IntoResponse, Extension, Json};

use crate::{AppError, AppState, User, WorkSpace};

pub(crate) async fn list_chat_users_handler(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, AppError> {
    let users = WorkSpace::fetch_all_chat_users(user.ws_id as _, &state.pool).await?;
    Ok(Json(users))
}
