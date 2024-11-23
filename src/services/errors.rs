use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum UserCreationError {
    UserAlreadyExists,
    GenericError,
}

impl IntoResponse for UserCreationError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            UserCreationError::GenericError => {
                (StatusCode::BAD_REQUEST, "Generic Error".to_string())
            }
            UserCreationError::UserAlreadyExists => {
                (StatusCode::BAD_REQUEST, "User already exists".to_string())
            }
        };
        (status, body).into_response()
    }
}
