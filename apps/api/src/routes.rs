use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, rejection::JsonRejection},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Serialize;

use crate::tts::{self, GenerateRequest};

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

pub(crate) fn router() -> Router {
    Router::new().route("/health", get(health)).route(
        "/api/tts/generate",
        post(generate).layer(DefaultBodyLimit::max(16 * 1024)),
    )
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: &'static str,
}

fn error(status: StatusCode, code: &'static str, message: &'static str) -> Response {
    (
        status,
        Json(ErrorResponse {
            error: ErrorDetail { code, message },
        }),
    )
        .into_response()
}

async fn generate(payload: Result<Json<GenerateRequest>, JsonRejection>) -> Response {
    let request = match payload {
        Ok(Json(request)) => request,
        Err(rejection) if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE => {
            return error(
                StatusCode::PAYLOAD_TOO_LARGE,
                "request_too_large",
                "Le corps de la requête est limité à 16 Kio.",
            );
        }
        Err(_) => {
            return error(
                StatusCode::BAD_REQUEST,
                "invalid_json",
                "JSON invalide : text et voice_id sont des chaînes ; speed, stability et similarity sont des nombres obligatoires. Utilisez Content-Type: application/json et aucun champ supplémentaire.",
            );
        }
    };
    match request.validate() {
        Ok(request) => (
            [
                (header::CONTENT_TYPE, "audio/wav"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"labonneia-test.wav\"",
                ),
                (header::CACHE_CONTROL, "no-store"),
            ],
            tts::generate(request),
        )
            .into_response(),
        Err(invalid) => error(StatusCode::BAD_REQUEST, invalid.code, invalid.message),
    }
}
