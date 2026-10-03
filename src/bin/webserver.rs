use axum::{
    routing::post,
    Json, Router,
};
use serde_json::Value;

async fn receive_json(Json(data): Json<Value>) -> &'static str {
    println!("Received JSON");

    // process/store data...

    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/data", post(receive_json));

    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:8000"
    )
    .await
    .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}