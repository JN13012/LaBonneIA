use axum::{body::Body, http::Request, http::StatusCode};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn health_returns_ok_json() {
    let response = labonneia_api::app()
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "application/json");
    let body = axum::body::to_bytes(response.into_body(), 1024)
        .await
        .unwrap();
    let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload, json!({ "status": "ok" }));
}
