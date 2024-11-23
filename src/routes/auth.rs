use crate::{
    services::{errors::UserCreationError, user_service::UserService},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iat: usize,
    exp: usize,
    pub sub: String,
}

const AUTH_TOKEN: &str = "CHANGEMEIAMONLYHARDCODED";

#[derive(Deserialize, Serialize)]
pub struct UserLoginData {
    pub user: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
    token: Option<String>,
}

pub async fn login(
    State(state): State<AppState>,
    Json(login_data): Json<UserLoginData>,
) -> impl IntoResponse {
    let db = &state.db_conn;
    let user = UserService::new(db.clone())
        .get_user_by_id(&login_data.user)
        .await;

    match user {
        Ok(user) => {
            if let Some(u) = user {
                let now = chrono::prelude::Utc::now();
                let delta = chrono::TimeDelta::days(1);
                let exp = now + delta;
                let claims = Claims {
                    iat: now.timestamp() as usize,
                    exp: exp.timestamp() as usize,
                    sub: u.username,
                };
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(AUTH_TOKEN.as_bytes()),
                );
                return (
                    StatusCode::OK,
                    Json(LoginResponse {
                        message: "Successfully logged in".to_owned(),
                        token: Some(token.unwrap()),
                    }),
                );
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(LoginResponse {
                        message: "User not found".to_owned(),
                        token: None,
                    }),
                );
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LoginResponse {
                    message: "Internal server error".to_owned(),
                    token: None,
                }),
            )
        }
    }
}

pub async fn signup(
    State(app_state): State<AppState>,
    user_data: Json<UserLoginData>,
) -> impl IntoResponse {
    let db = app_state.db_conn;
    let user_creation = UserService::new(db.clone()).create_user(&user_data).await;
    match user_creation {
        Ok(()) => return (StatusCode::OK, "User created").into_response(),

        Err(UserCreationError::GenericError) => {
            return UserCreationError::GenericError.into_response()
        }
        Err(UserCreationError::UserAlreadyExists) => {
            return UserCreationError::UserAlreadyExists.into_response()
        }
    }
}
