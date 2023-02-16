use std::collections::HashMap;
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    let data = r#"
    {
        "model": "text-davinci-003",
        "prompt": "Describe the Rust programming language",
        "temperature": 0.9,
        "max_tokens": 1024
    }"#;

    let mut map = HashMap::new();
    map.insert("model", "text-davinci-003");
    map.insert("prompt", "describe rust programming language");
    map.insert("temperature", "0.9");
    map.insert("max_tokens", "1024");

    let args: Vec<String> = env::args().collect();
    let auth_token = &args[1];

    println!("Auth token: {}", auth_token);
    let bearer_auth = format!("Bearer {}", auth_token);
    println!("Bearer Auth token: {}", bearer_auth);

    let url = "https://api.openai.com/v1/completions".to_string();
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(ACCEPT, "*/*")
        .header(AUTHORIZATION, &bearer_auth)
        .header(CONTENT_TYPE, "application/json")
        .body(data)
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("🔥 Response: {:#?}", response);
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: [{:#?}]", other);
        }
    };
}