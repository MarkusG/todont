use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::{Claims, RegisteredClaims, SignWithKey, VerifyWithKey};
use axum::{
    async_trait,
    extract::FromRequestParts,
    response::{Response, IntoResponse},
    TypedHeader,
    headers::Authorization,
    headers::authorization::Bearer,
    http::{StatusCode, request::Parts},
    RequestPartsExt
};

use crate::models::permissions::Permissions;

pub struct ApplicationClaims {
    pub username: String,
    pub permissions: Permissions
}

#[async_trait]
impl<S> FromRequestParts<S> for ApplicationClaims
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

        Ok(ApplicationClaims {
            username: claims.private["username"]
                .as_str().expect("username was not a string").to_string(),
            permissions: claims.private["permissions"]
                .as_i64().expect("permissions was not an i64") })
    }
}

pub enum AuthError {
    InvalidToken
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Invalid token").into_response()
    }
}

pub fn generate_token(claims: ApplicationClaims) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"1234").unwrap();

    let mut private = BTreeMap::new();
    private.insert("username".to_string(), claims.username.into());
    private.insert("permissions".to_string(), claims.permissions.into());

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

    claims.sign_with_key(&key).unwrap()
}
