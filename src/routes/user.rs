use crate::middleware::{get_token, verify_token};
use axum::{extract::Request, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn get_user_info(req: Request) -> impl IntoResponse {
    let headers = req.headers();
    let jwt = get_token(headers).unwrap();
    let decoded_token = verify_token(jwt).unwrap();
    let claims = decoded_token.claims;
    (
        StatusCode::OK,
        Json(json!({"message":format!("{}, {}","Welcome",claims.sub)})),
    ).into_response()
}
