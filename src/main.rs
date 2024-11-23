mod middleware;
mod model;
mod routes;
mod services;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use routes::{auth, user};

use sea_orm::{Database, DatabaseConnection, DbErr};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use dotenvy::from_path;
#[derive(Clone)]
struct AppState {
    db_conn: Arc<DatabaseConnection>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    from_path("./.env").unwrap();

    let db_conn = init_db().await?;
    let app_state = AppState {
        db_conn: Arc::new(db_conn),
    };

    let app = Router::new()
        .route("/protected/user", get(user::get_user_info))
        .route_layer(axum::middleware::from_fn(middleware::jwt_middleware))
        .route("/signup", post(auth::signup))
        .route("/login", post(auth::login))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

async fn init_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(std::env::var("DATABASE_URL").unwrap()).await?;
    Ok(db)
}
