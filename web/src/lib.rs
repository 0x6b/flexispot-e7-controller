use std::{error::Error, fmt::Display};

use flexispot_e7_controller_lib::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RequestPayload {
    pub command: Command,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponsePayload {
    Message(String),
    Height(i32),
}

impl Display for ResponsePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponsePayload::Message(s) => write!(f, "{s}"),
            ResponsePayload::Height(h) => write!(f, "{h}"),
        }
    }
}

impl From<&str> for ResponsePayload {
    fn from(s: &str) -> Self {
        ResponsePayload::Message(s.to_string())
    }
}

impl From<Box<dyn Error>> for ResponsePayload {
    fn from(e: Box<dyn Error>) -> Self {
        ResponsePayload::Message(e.to_string())
    }
}

impl From<i32> for ResponsePayload {
    fn from(i: i32) -> Self {
        ResponsePayload::Height(i)
    }
}
