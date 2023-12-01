use axum::{extract::Path, routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/1/*path", get(pow))
}

async fn pow(Path(p): Path<String>) -> String {
    p.split('/')
        .map(|c| c.parse::<i32>().unwrap())
        .reduce(|a, b| a ^ b)
        .map(|x| x.pow(3).to_string())
        .unwrap()
}
