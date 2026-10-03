use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let data = json!({
        "message": "Hello from the client",
    });

    let response = client
        .post("http://127.0.0.1:9000/data")
        .json(&data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Transfer successful");
    } else {
        eprintln!("Transfer failed: {}", response.status());
    }

    Ok(())
}
