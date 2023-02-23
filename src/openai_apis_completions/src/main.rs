/*
   Copyright (c) 2023
   Author      : Bruno Capuano
   Create Time : 2023 Feb
   Change Log  :
   
   The MIT License (MIT)
   Permission is hereby granted, free of charge, to any person obtaining a copy
   of this software and associated documentation files (the "Software"), to deal
   in the Software without restriction, including without limitation the rights
   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
   copies of the Software, and to permit persons to whom the Software is
   furnished to do so, subject to the following conditions:
   The above copyright notice and this permission notice shall be included in
   all copies or substantial portions of the Software.
   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
   THE SOFTWARE.
*/

use std::collections::HashMap;
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub text: String,
    pub index: i64,
    pub logprobs: Value,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let auth_token = &args[1];
    let prompt = &args[2];

    let mut data: String = r#"
    {
        "model": "text-davinci-003",
        "prompt": "{}",
        "temperature": 0.9,
        "max_tokens": 1024
    }"#.to_string();
    data = format! {"{}", data.replace("{}", prompt)};

    let mut map = HashMap::new();
    map.insert("model", "text-davinci-003");
    map.insert("prompt", prompt);
    map.insert("temperature", "0.9");
    map.insert("max_tokens", "1024");

    println!("Auth token: {}", auth_token);
    let bearer_auth = format!("Bearer {}", auth_token);
    println!("Prompt: {}", prompt);

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

            match response.json::<Root>().await {
                Ok(parsed) => {
                    println!("🔥 Success!");
                    println!("💬 Response: {}", parsed.choices[0].text);
                },
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };

        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Status: UNAUTHORIZED - Need to grab a new token");
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            println!("Status: 429 - Too many requests");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: [{:#?}]", other);
        }
    };
}