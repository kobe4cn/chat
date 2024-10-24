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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AppConfig, User};
    use anyhow::Result;

    use axum::{body::Body, http::status, middleware::from_fn_with_state, routing::get, Router};

    use tower::ServiceExt;

    async fn handler() -> impl IntoResponse {
        (StatusCode::OK, "ok").into_response()
    }

    #[tokio::test]
    async fn test_verify_token_should_work() -> Result<()> {
        let config = AppConfig::try_load()?;
        let state = AppState::try_new(config).await?;
        let user = User::new(1, "test", "test@163.com");
        let token = state.ek.sign(user)?;
        let app = Router::new()
            .route("/", get(handler))
            .layer(from_fn_with_state(state.clone(), verify_token))
            .with_state(state);

        let req = Request::builder()
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())?;
        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), status::StatusCode::OK);

        Ok(())
    }
}
