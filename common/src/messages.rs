pub mod handlers;
pub mod common_crypto;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageError {
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub req_type: String,
    pub data: serde_json::Value
}