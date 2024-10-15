use axum::{
    extract::{FromRequestParts, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use tracing::warn;

use crate::AppState;

pub async fn verify_token(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let (mut parts, body) = req.into_parts();
    match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state).await {
        Ok(TypedHeader(Authorization(bearer))) => {
            let token = bearer.token();
            match state.dk.verify(token) {
                Ok(user) => {
                    let mut req = Request::from_parts(parts, body);
                    req.extensions_mut().insert(user);
                    next.run(req).await
                }
                Err(e) => {
                    let msg = format!("verify token failed: {}", e);
                    warn!(msg);
                    (StatusCode::FORBIDDEN, msg).into_response()
                }
            }
        }
        Err(e) => {
            let msg = format!("parse Authorization header failed: {}", e);
            warn!(msg);
            (StatusCode::UNAUTHORIZED, msg).into_response()
        }
    }
}
