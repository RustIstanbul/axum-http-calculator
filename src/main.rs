use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = build_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn build_app() -> Router {
    Router::new()
        .route("/add", post(calculate_add))
        .route("/subtract", post(calculate_subtract))
        .route("/multiply", post(calculate_multiply))
        .route("/divide", post(calculate_divide))
}

async fn calculate_add(Json(payload): Json<Numbers>) -> impl IntoResponse {
    let res = CalculationResult {
        number: payload.number1 + payload.number2,
    };
    (StatusCode::OK, Json(res))
}

async fn calculate_subtract(Json(payload): Json<Numbers>) -> impl IntoResponse {
    let res = CalculationResult {
        number: payload.number1 - payload.number2,
    };
    (StatusCode::OK, Json(res))
}

async fn calculate_divide(Json(payload): Json<Numbers>) -> impl IntoResponse {
    let res = CalculationResult {
        number: payload.number1 / payload.number2,
    };
    (StatusCode::OK, Json(res))
}

async fn calculate_multiply(Json(payload): Json<Numbers>) -> impl IntoResponse {
    let res = CalculationResult {
        number: payload.number1 * payload.number2,
    };
    (StatusCode::OK, Json(res))
}

#[derive(Serialize, Deserialize)]
struct Numbers {
    number1: f64,
    number2: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CalculationResult {
    number: f64,
}

#[cfg(test)]
mod tests {
    use super::{build_app, CalculationResult};
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
    };
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[tokio::test]
    async fn add() {
        let body_json = json!({
            "number1": 5,
            "number2": 5,
        });
        let result = CalculationResult { number: 10.0 };

        test_route("/add", body_json, result).await;
    }

    #[tokio::test]
    async fn subtract() {
        let body_json = json!({
            "number1": 10,
            "number2": 5,
        });
        let result = CalculationResult { number: 5.0 };

        test_route("/subtract", body_json, result).await;
    }

    #[tokio::test]
    async fn multiply() {
        let body_json = json!({
            "number1": 5,
            "number2": 5,
        });
        let result = CalculationResult { number: 25.0 };

        test_route("/multiply", body_json, result).await;
    }

    #[tokio::test]
    async fn divide() {
        let body_json = json!({
            "number1": 20,
            "number2": 4,
        });
        let result = CalculationResult { number: 5.0 };

        test_route("/divide", body_json, result).await;
    }

    async fn test_route(route: &'static str, body_json: Value, result: CalculationResult) {
        let app = build_app();

        let body = Body::from(serde_json::to_string(&body_json).unwrap());

        let request = Request::builder()
            .method(Method::POST)
            .uri(route)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(body)
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_result: CalculationResult = serde_json::from_slice(&body).unwrap();

        assert_eq!(response_result, result);
    }
}
