
use crate::middleware::{get_token, verify_token};
use crate::services::file_service::{self, FileService};
use crate::AppState;
use axum::body::{Body, BodyDataStream};
use axum::extract::{Path, Request, State, Multipart};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::debug_handler;
#[debug_handler]
pub async fn get_files_by_folder(
    State(app_state): State<AppState>,
    Path(folder): Path<String>,
    req: Request<Body>
) -> impl IntoResponse {
    let file_service = FileService {
        db: app_state.db_conn,
    };
    let headers = req.headers();
    let jwt = get_token(headers).unwrap();
    let decoded_token = verify_token(jwt).unwrap();
    let claims = decoded_token.claims;

    let user = claims.sub;
    file_service.get_files_by_folder(&user, &folder).await;
    (StatusCode::OK, "Ok").into_response()
}

pub async fn upload(State(app_state): State<AppState>,mut file: Multipart) -> impl IntoResponse 
{
    let file_service = FileService {
        db: app_state.db_conn
    };
    file_service.upload(file).await;


}