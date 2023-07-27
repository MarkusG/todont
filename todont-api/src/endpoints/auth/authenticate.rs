use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::auth::{generate_token, ApplicationClaims};
use crate::models::AuthenticateRequest;
use crate::models::permissions::*;

pub async fn authenticate(Json(body): Json<AuthenticateRequest>) -> impl IntoResponse {
    if body.username.is_empty() || body.password.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    if body.username != "mark" || body.password != "1234" {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let token = generate_token(ApplicationClaims {
        username: "mark".to_string(),
        permissions: CREATE_TODO | UPDATE_TODO | READ_TODOS | DELETE_TODO
    });

    (StatusCode::OK, token).into_response()
}

pub async fn authorized(claims: ApplicationClaims) -> impl IntoResponse {
    println!("username: {}", claims.username);
    println!("permissions: {}", claims.permissions);
}
