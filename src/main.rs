use axum::{
    extract::{Json, Request, rejection::JsonRejection, DefaultBodyLimit},
    http::StatusCode,
    middleware::{from_fn, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::VecDeque;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    sensor_id: String,
    value: f64,
    timestamp: String,
}

#[derive(Debug, Serialize)]
struct SuccessResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/data", post(handle_data))
        .layer(DefaultBodyLimit::max(16 * 1024))
        .layer(from_fn(layer_log));

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("failed to bind 0.0.0.0:3000: {err}");
            return Err(err);
        }
    };

    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("server error: {err}");
        return Err(err);
    }

    Ok(())
}

async fn handle_data(payload: Result<Json<RequestPayload>, JsonRejection>) -> Response {
    match payload {
        Ok(Json(payload)) => {
            println!("{} {} {}", payload.sensor_id, payload.value, payload.timestamp);
            return validate_data(&payload)
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                Json(ErrorResponse {
                    error: "missing or invalid Content-Type, expected application/json".to_string(),
                }),
            )
                .into_response()
        }
        Err(JsonRejection::JsonDataError(_)) => {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "JSON shape/type mismatch for RequestPayload".to_string(),
                }),
            )
                .into_response()
        }
        Err(JsonRejection::JsonSyntaxError(_)) => {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "invalid JSON syntax".to_string(),
                }),
            )
                .into_response()
        }
        Err(JsonRejection::BytesRejection(_)) => {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "failed to read request body".to_string(),
                }),
            )
                .into_response()
        }
        Err(_) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "unexpected JSON extraction error".to_string(),
                }),
            )
                .into_response()
        }
    }
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

fn parse_timestamp_strict(input: &str) -> Result<DateTime<Utc>, ()> {
    let naive =
        NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%SZ").map_err(|_| ())?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
}

fn validate_data(payload: &RequestPayload) -> Response {
    match parse_timestamp_strict(&payload.timestamp) {
        Ok(ts) => {
            return(
                StatusCode::OK,
                Json(SuccessResponse {
                    message: format!("payload accepted for sensor {}", payload.sensor_id),
                }),
            ).into_response();
        }
        Err(_) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse {
                    error: "timestamp must match YYYY-MM-DDTHH:MM:SSZ".to_string(),
                }),
            )
                .into_response();
        }
    }
}