pub mod client;
pub mod server;

use std::collections::HashMap;
use serde_json::to_string;

pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: &str, message: &str) -> ErrorResponse {
        ErrorResponse {
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn response(&self) -> String {
        let mut map = HashMap::new();
        map.insert("status", self.code.clone());
        map.insert("message", self.message.clone());
        to_string(&map).unwrap()
    }
}
