use axum::{response::Html, routing::get, Router};
mod db;

mod models;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { Html("Connected to the backend!") }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2121").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
