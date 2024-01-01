use std::{
    error::Error,
    sync::{Arc, RwLock},
};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use flexispot_e7_controller_lib::{Command, FlexispotE7Controller};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct RequestPayload {
    command: Command,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum ResponsePayload {
    Message(String),
    Height(i32),
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

#[derive(Clone)]
struct AppState {
    controller: Arc<RwLock<FlexispotE7Controller>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = AppState {
        controller: Arc::new(RwLock::new(FlexispotE7Controller::try_new_with("/dev/ttyS0", 12)?)),
    };

    let app = Router::new()
        .route("/query", get(query_handler))
        .route("/", post(post_handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn query_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut controller = state.controller.write().unwrap();
    match (*controller).query() {
        Ok(height) => (StatusCode::OK, Json(ResponsePayload::from(height))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponsePayload::from(e))),
    }
}

async fn post_handler(
    State(state): State<AppState>,
    Json(payload): Json<RequestPayload>,
) -> impl IntoResponse {
    let command = payload.command;
    if command == Command::Query || command == Command::WakeUp || command == Command::Memory {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponsePayload::from("query, wakeup, or memory command is not allowed")),
        );
    }

    let mut controller = state.controller.write().unwrap();
    match (*controller).execute(&command) {
        Ok(_) => (StatusCode::OK, Json(ResponsePayload::from("ok"))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponsePayload::from(e))),
    }
}
