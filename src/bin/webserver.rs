use axum::{
    routing::post,
    Json, Router,
};
use serde_json::{json, Value};

async fn receive_json(Json(_data): Json<Value>) -> Json<Value> {
    println!("Received JSON");

    // process/store data...

    Json(json!({
        "status": "OK"
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/data", post(receive_json));

    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:9000"
    )
    .await
    .unwrap();

    axum::serve(listener, app)
        .await
        ?;
    Ok(())
}
