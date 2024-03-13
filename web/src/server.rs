#[cfg(all(target_os = "linux", target_arch = "arm"))]
use {
    axum::{
        extract::{Request, State},
        http::{header::AUTHORIZATION, StatusCode},
        middleware::{from_fn_with_state, Next},
        response::{IntoResponse, Response},
        routing::{get, post},
        serve, Json, Router,
    },
    clap::Parser,
    flexispot_e7_controller_lib::{Command, Controller},
    flexispot_e7_controller_web::{RequestPayload, ResponsePayload},
    serde::Deserialize,
    std::{
        fs::read_to_string,
        net::IpAddr,
        path::PathBuf,
        sync::{Arc, RwLock},
    },
    tracing::info,
};

#[cfg(all(target_os = "linux", target_arch = "arm"))]
#[derive(Clone)]
struct AppState {
    controller: Arc<RwLock<Controller>>,
    secret: String,
}

#[cfg(all(target_os = "linux", target_arch = "arm"))]
#[derive(Debug, Parser)]
#[clap(about, version)]
pub struct Args {
    #[clap(short, long, default_value = "config.toml")]
    config: PathBuf,
}

#[cfg(all(target_os = "linux", target_arch = "arm"))]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Config {
    /// Path to the serial device
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "linux", target_arch = "arm"))]
    {
        tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
        let args = Args::parse();
        let Config { device, pin20, secret, address, port } =
            toml::from_str(&(read_to_string(args.config)?))?;

        info!("{address}:{port}");

        let state = AppState {
            controller: Arc::new(RwLock::new(Controller::try_new_with(device, pin20)?)),
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
    }
    Ok(())
}

#[cfg(all(target_os = "linux", target_arch = "arm"))]
async fn auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .filter(|v| *v == state.secret)
    {
        Some(_) => Ok(next.run(req).await),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

#[cfg(all(target_os = "linux", target_arch = "arm"))]
async fn query_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut controller = state.controller.write().unwrap();
    match (*controller).query() {
        Ok(height) => {
            info!("/query: {height}cm");
            (StatusCode::OK, Json(ResponsePayload::from(height)))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponsePayload::from(e))),
    }
}

#[cfg(all(target_os = "linux", target_arch = "arm"))]
async fn post_handler(
    State(state): State<AppState>,
    Json(payload): Json<RequestPayload>,
) -> impl IntoResponse {
    info!("{:?}", payload);
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
