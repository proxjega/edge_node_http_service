use axum::{
    extract::{Json, Request, rejection::JsonRejection},
    middleware::{from_fn, Next},
    response::Response,
    routing::post,
    Router,
};
use serde::Deserialize;
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct RequestPayload {
    sensor_id: String,
    value: f64,
    timestamp: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/data", post(handle_data))
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

async fn handle_data(payload: Result<Json<RequestPayload>, JsonRejection>) {
    match payload {
        Ok(payload) => {
            println!("{} {} {}", payload.sensor_id, payload.value, payload.timestamp)
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            // Request didn't have `Content-Type: application/json`
            // header
        }
        Err(JsonRejection::JsonDataError(_)) => {
            // Couldn't deserialize the body into the target type
        }
        Err(JsonRejection::JsonSyntaxError(_)) => {
            // Syntax error in the body
        }
        Err(JsonRejection::BytesRejection(_)) => {
            // Failed to extract the request body
        }
        Err(_) => {
            // `JsonRejection` is marked `#[non_exhaustive]` so match must
            // include a catch-all case.
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