#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Request {
    Subscribe(String),
    Send { channel: String, message: String },
    Count(),
    Control(Value),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Reply {
    Subscribed(String),
    Error(String),
    Count(usize),
    Control(Value),
    Message { channel: String, message: String },
}
