use axum::{
    Router, 
    body::{Body, to_bytes}, 
    extract::Request, 
    middleware::{Next, from_fn}, 
    response::Response, 
    routing::post
};
use std::time::Instant;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // build our application with a single route
    let app = Router::new()
    .route("/data", post(|| async { "Post detected" }))
    .layer(from_fn(layer_log));

    // run our app with hyper, listening globally on port 3000
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