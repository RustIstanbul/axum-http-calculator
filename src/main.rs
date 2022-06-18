use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/add", post(calculate_add));
        
        
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn calculate_add(
    Json(payload): Json<Numbers>,
) -> impl IntoResponse {
    let res = CalculationResult {
      number: payload.number1 + payload.number2
    };
    (StatusCode::OK, Json(res))
}


#[derive(Serialize,Deserialize)]
struct Numbers {
    number1: f64,
    number2: f64,
}

#[derive(Serialize,Deserialize)]
struct CalculationResult {
    number: f64
}