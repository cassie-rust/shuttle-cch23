use axum::{http::StatusCode, routing::get, Router};

async fn ok() -> StatusCode {
    StatusCode::OK
}

async fn error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(ok))
        .route("/-1/error", get(error))
}
