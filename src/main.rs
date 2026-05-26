use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(hello))
        .route("/echo", post(echo));

    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello from Axum!"
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

#[derive(Deserialize)]
struct EchoIn {
    message: String,
}

#[derive(Serialize)]
struct EchoOut {
    echoed: String,
}

async fn echo(Json(payload): Json<EchoIn>) -> Json<EchoOut> {
    Json(EchoOut {
        echoed: payload.message,
    })
}
