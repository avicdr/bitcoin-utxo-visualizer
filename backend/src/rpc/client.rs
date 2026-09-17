#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct BitcoinRpcClient {
    pub url: String,
    pub user: String,
    pub pass: String,
    pub client: reqwest::Client,
}

#[derive(Serialize)]
struct JsonRpcRequest<'a, T> {
    jsonrpc: &'static str,
    id: &'static str,
    method: &'a str,
    params: T,
}

#[derive(Deserialize, Debug)]
pub struct JsonRpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
}

#[derive(Deserialize, Debug)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

impl BitcoinRpcClient {
    pub fn new(url: String, user: String, pass: String) -> Self {
        Self {
            url,
            user,
            pass,
            client: reqwest::Client::new(),
        }
    }
}
