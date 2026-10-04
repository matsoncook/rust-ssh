use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let data = json!({
        "message": "Hello from the client",
    });

    let response = client
        .post("http://127.0.0.1:8000/data")
        .json(&data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Transfer successful");
        let json : Value = response.json().await.unwrap();
        println!("Response JSON: {:?}", json);
    } else {
        eprintln!("Transfer failed: {}", response.status());
    }

    Ok(())
}
