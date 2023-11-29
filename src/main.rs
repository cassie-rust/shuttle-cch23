use axum::Router;

mod calendar;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest("/", calendar::router());

    Ok(router.into())
}
