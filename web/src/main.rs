use std::{
    error::Error,
    fs::read_to_string,
    net::IpAddr,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    serve, Json, Router,
};
use clap::Parser;
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
    secret: String,
}

#[derive(Debug, Parser)]
#[clap(about, version)]
pub struct Args {
    #[clap(short, long, default_value = "config.toml")]
    config: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Config {
    /// Path to serial device
    pub device: PathBuf,

    /// GPIO (BCM) number of PIN 20
    pub pin20: u8,

    /// Authentication secret
    pub secret: String,

    /// IP address to bind
    pub address: IpAddr,

    /// Port number to bind
    pub port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let Config { device, pin20, secret, address, port } =
        toml::from_str(&(read_to_string(args.config)?))?;

    let state = AppState {
        controller: Arc::new(RwLock::new(FlexispotE7Controller::try_new_with(device, pin20)?)),
        secret,
    };

    let app = Router::new()
        .route("/query", get(query_handler))
        .route("/", post(post_handler))
        .route_layer(from_fn_with_state(state.clone(), auth))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}"))
        .await
        .unwrap();

    serve(listener, app).await.unwrap();
    Ok(())
}

async fn auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let header = match req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
    {
        Some(v) => v,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if header == state.secret {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
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
