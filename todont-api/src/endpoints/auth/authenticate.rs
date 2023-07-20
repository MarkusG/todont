use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use jwt::{Claims, RegisteredClaims, SignWithKey, VerifyWithKey};
use sha2::Sha256;

use axum::http::StatusCode;
use axum::{Json, RequestPartsExt};
use axum::response::IntoResponse;

use axum::response::Response;
use axum::async_trait;
use axum::http::request::Parts;
use axum::extract::FromRequestParts;
use axum::TypedHeader;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;

use crate::models::AuthenticateRequest;

pub async fn authenticate(Json(body): Json<AuthenticateRequest>) -> impl IntoResponse {
    if body.username.is_empty() || body.password.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    if body.username != "mark" || body.password != "1234" {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"1234").unwrap();

    let mut private = BTreeMap::new();
    private.insert("username".to_string(), "mark".into());

    let now = SystemTime::now();
    let unix_ts = now
        .duration_since(UNIX_EPOCH)
        .expect("Clara, get the TARDIS")
        .as_secs();

    let claims = Claims {
        registered: RegisteredClaims {
            issuer: None,
            subject: None,
            audience: None,
            expiration: Some(unix_ts + 10),
            not_before: Some(unix_ts),
            issued_at: Some(unix_ts),
            json_web_token_id: None
        },
        private
    };

    let token = claims.sign_with_key(&key).unwrap();

    (StatusCode::OK, token).into_response()
}

pub async fn authorized(claims: MyClaims) -> impl IntoResponse {
    println!("{}", claims.username);
}

#[async_trait]
impl<S> FromRequestParts<S> for MyClaims
    where S: Send + Sync
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| { println!("{:?}", e); AuthError::InvalidToken })?;

        let key: Hmac<Sha256> = Hmac::new_from_slice(b"1234").unwrap();
        let claims: Claims = bearer.token()
            .verify_with_key(&key)
            .map_err(|e| { println!("{:?}", e); AuthError::InvalidToken })?;

        Ok(MyClaims { username: claims.private["username"].to_string() })
    }
}

pub struct MyClaims {
    username: String
}

pub enum AuthError {
    InvalidToken
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Invalid token").into_response()
    }
}
