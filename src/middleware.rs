use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::routes::auth::Claims;

const SECRET: &str = "CHANGEMEIAMONLYHARDCODED";

pub async fn jwt_middleware(request: Request, next: Next) -> Response {
    let headers = request.headers();
    if let Some(token) = get_token(headers) {
        let decoded_token = verify_token(token);
        match decoded_token {
            Ok(_) => {
                return next.run(request).await;
            }
            Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
        }
    } else {
        return StatusCode::UNAUTHORIZED.into_response();
    }
}

pub fn get_token(headers: &HeaderMap) -> Option<&str> {
    if let Some(t) = headers.get("Authorization") {
        let token = t.to_str();
        match token {
            Ok(token) => return Some(token),
            Err(_) => return None,
        }
    }
    None
}

pub fn verify_token(
    token: &str,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let decoded_token: Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> =
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        );
    decoded_token
}
