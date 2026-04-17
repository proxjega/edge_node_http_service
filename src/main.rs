use axum::{
    extract::{DefaultBodyLimit, Json, Request, State, rejection::JsonRejection},
    http::StatusCode,
    middleware::{from_fn, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Arc, time::Instant};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    sensor_id: String,
    value: f64,
    timestamp: String,
}

#[derive(Debug, Serialize)]
struct SuccessResponse {
    moving_average: f64,
    timesamp: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Clone)]
struct AppState {
    values: Arc<Mutex<VecDeque<f64>>>,
    min_threshold: f64,
    max_threshold: f64,
}

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: u16,
    min_threshold: f64,
    max_threshold: f64,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    const REQUEST_SIZE_LIMIT : usize = 16384;
    
    let config = match load_config("config.json") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("configuration error: {err}");
            return Err(err);
        }
    };

    println!(
        "loaded config: bind {}:{}, thresholds {}..{}",
        config.ip, config.port, config.min_threshold, config.max_threshold
    );
    
    let state = AppState {
        values: Arc::new(Mutex::new(VecDeque::new())),
        min_threshold: config.min_threshold,
        max_threshold: config.max_threshold
    };

    let app = Router::new()
        .route("/data", post(handle_post_data))
        .layer(DefaultBodyLimit::max(REQUEST_SIZE_LIMIT))
        .layer(from_fn(layer_log))
        .with_state(state);

    let bind_addr = format!("{}:{}", config.ip, config.port);
    let listener = match tokio::net::TcpListener::bind(&bind_addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("failed to bind {bind_addr}: {err}");
            return Err(err);
        }
    };

    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("server error: {err}");
        return Err(err);
    }

    Ok(())
}

fn load_config(path: &str) -> std::io::Result<Config> {
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            println!("config file not found: {path}");
            return Err(err);
        }
        Err(err) => {
            println!("failed to read config file {path}: {err}");
            return Err(err);
        }
    };

    match serde_json::from_str::<Config>(&contents) {
        Ok(config) => {
            if config.ip.trim().is_empty() {
                println!("config error: missing ip value");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "missing ip value",
                ));
            }

            if config.port == 0 {
                println!("config error: missing or invalid port value");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "missing or invalid port value",
                ));
            }

            if config.min_threshold > config.max_threshold {
                println!("config error: min_threshold is greater than max_threshold");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "min_threshold is greater than max_threshold",
                ));
            }

            Ok(config)
        }
        Err(err) => {
            println!("config parse error in {path}: {err}");
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
        }
    }
}

async fn handle_post_data(state: State<AppState>, payload: Result<Json<RequestPayload>, JsonRejection>) -> Response {
    let payload = match decode_json(payload) {
        Ok(payload) => payload,
        Err(response) => return response,
    };

    if let Err(response) = validate_data(&state, &payload) {
        return response;
    }

    let success = process_data( state, payload).await;
    (StatusCode::OK, Json(success)).into_response()
}

async fn layer_log(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start = Instant::now();
    println!("[REQ] {} {}", method, path);

    let res = next.run(req).await;
    let status = res.status();
    let elapsed_ms = start.elapsed().as_micros();

    println!("[RES] {} {} -> {} ({} micros)", method, path, status, elapsed_ms);

    res
}

fn parse_timestamp_strict(input: &str) -> Result<(), ()> {
    let _naive =
        NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%SZ").map_err(|_| ())?;
    Ok(())
}

fn decode_json(payload: Result<Json<RequestPayload>, JsonRejection>) -> Result<RequestPayload, Response> {
    match payload {
        Ok(Json(payload)) => Ok(payload),
        Err(JsonRejection::MissingJsonContentType(_)) => Err(
            (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                Json(ErrorResponse {
                    error: "missing or invalid Content-Type, expected application/json".to_string(),
                }),
            )
                .into_response(),
        ),
        Err(JsonRejection::JsonDataError(_)) => Err(
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "JSON shape/type mismatch for RequestPayload".to_string(),
                }),
            )
                .into_response(),
        ),
        Err(JsonRejection::JsonSyntaxError(_)) => Err(
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "invalid JSON syntax".to_string(),
                }),
            )
                .into_response(),
        ),
        Err(JsonRejection::BytesRejection(_)) => Err(
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "failed to read request body".to_string(),
                }),
            )
                .into_response(),
        ),
        Err(_) => Err(
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "unexpected JSON extraction error".to_string(),
                }),
            )
                .into_response(),
        ),
    }
}

fn validate_data(state: &State<AppState>, payload: &RequestPayload) -> Result<(), Response> {
    if parse_timestamp_strict(&payload.timestamp).is_err() {
        return Err(
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse {
                    error: "timestamp must match YYYY-MM-DDTHH:MM:SSZ".to_string(),
                }),
            )
                .into_response(),
        );
    }

    if payload.value < state.min_threshold || payload.value > state.max_threshold {
        return Err(
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse {
                    error: format!(
                        "value must be between {} and {}",
                        state.min_threshold, state.max_threshold
                    ),
                }),
            )
                .into_response(),
        );
    }

    Ok(())
}

async fn process_data(state: State<AppState>, payload: RequestPayload) -> SuccessResponse {
    // println!("{} {} {}", payload.sensor_id, payload.value, payload.timestamp);

    let mut values = state.values.lock().await;
    values.push_back(payload.value);
    if values.len() > 10 {
        values.pop_front();
    }

    let avg = values.iter().sum::<f64>() / values.len() as f64;
    // println!("moving average: {}", avg);

    SuccessResponse {
        moving_average: avg,
        timesamp: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    }
}