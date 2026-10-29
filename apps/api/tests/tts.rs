use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
    response::Response,
};
use serde_json::{Value, json};
use tower::ServiceExt;

fn valid_input() -> Value {
    json!({
        "text": "Bonjour, ceci est un test.",
        "voice_id": "mock-alba",
        "speed": 1.0,
        "stability": 0.5,
        "similarity": 0.75,
    })
}

async fn post(body: String, content_type: &str) -> Response {
    labonneia_api::app()
        .oneshot(
            Request::post("/api/tts/generate")
                .header("content-type", content_type)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap()
}

async fn expect_error(body: String, status: StatusCode, code: &str) {
    let response = post(body, "application/json").await;
    assert_eq!(response.status(), status);
    assert_eq!(response.headers()["content-type"], "application/json");
    let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
    let error: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(error["error"]["code"], code);
    assert!(!error["error"]["message"].as_str().unwrap().is_empty());
}

async fn wav(input: Value) -> Vec<u8> {
    let response = post(input.to_string(), "application/json").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "audio/wav");
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert_eq!(
        response.headers()["content-disposition"],
        "attachment; filename=\"labonneia-test.wav\""
    );
    to_bytes(response.into_body(), 100_000)
        .await
        .unwrap()
        .to_vec()
}

#[tokio::test]
async fn valid_request_returns_pcm_wav_with_audible_samples() {
    let bytes = wav(valid_input()).await;
    assert_eq!(&bytes[..4], b"RIFF");
    assert_eq!(&bytes[8..16], b"WAVEfmt ");
    assert_eq!(u16::from_le_bytes(bytes[20..22].try_into().unwrap()), 1); // PCM
    assert_eq!(u16::from_le_bytes(bytes[22..24].try_into().unwrap()), 1); // mono
    assert_eq!(
        u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
        16_000
    );
    assert_eq!(u16::from_le_bytes(bytes[34..36].try_into().unwrap()), 16);
    assert_eq!(&bytes[36..40], b"data");
    let riff_size = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let data_size = u32::from_le_bytes(bytes[40..44].try_into().unwrap()) as usize;
    assert_eq!(riff_size + 8, bytes.len());
    assert_eq!(data_size + 44, bytes.len());
    assert_eq!(data_size, 38_400);
    let peak = bytes[44..]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|sample| i16::from_le_bytes(*sample).unsigned_abs())
        .max()
        .unwrap();
    assert!((3_900..=4_000).contains(&peak));
}

#[tokio::test]
async fn mock_is_deterministic_and_ignores_text_voice_and_settings() {
    let reference = wav(valid_input()).await;
    assert_eq!(reference, wav(valid_input()).await);
    let other = json!({
        "text": "Un autre texte.", "voice_id": "mock-nova",
        "speed": 1.2, "stability": 0.0, "similarity": 1.0,
    });
    assert_eq!(reference, wav(other).await);
}

#[tokio::test]
async fn empty_or_whitespace_text_is_rejected() {
    for text in ["", " \n\t", "\u{2003}"] {
        let mut input = valid_input();
        input["text"] = json!(text);
        expect_error(input.to_string(), StatusCode::BAD_REQUEST, "empty_text").await;
    }
}

#[tokio::test]
async fn unicode_character_limit_is_inclusive_and_counts_untrimmed_text() {
    let mut input = valid_input();
    input["text"] = json!("🦀".repeat(2_000));
    wav(input.clone()).await;
    input["text"] = json!("🦀".repeat(2_001));
    expect_error(input.to_string(), StatusCode::BAD_REQUEST, "text_too_long").await;
    let mut input = valid_input();
    input["text"] = json!(format!(" {}", "x".repeat(2_000)));
    expect_error(input.to_string(), StatusCode::BAD_REQUEST, "text_too_long").await;
}

#[tokio::test]
async fn unknown_voice_is_rejected() {
    for voice in ["", "real-voice", "mock-alba "] {
        let mut input = valid_input();
        input["voice_id"] = json!(voice);
        expect_error(input.to_string(), StatusCode::BAD_REQUEST, "unknown_voice").await;
    }
}

#[tokio::test]
async fn out_of_range_settings_are_rejected() {
    for (field, values, code) in [
        ("speed", [0.69, 1.21], "invalid_speed"),
        ("stability", [-0.01, 1.01], "invalid_stability"),
        ("similarity", [-0.01, 1.01], "invalid_similarity"),
    ] {
        for value in values {
            let mut input = valid_input();
            input[field] = json!(value);
            expect_error(input.to_string(), StatusCode::BAD_REQUEST, code).await;
        }
    }
}

#[tokio::test]
async fn inclusive_setting_bounds_are_accepted() {
    for (speed, stability, similarity) in [(0.7, 0.0, 0.0), (1.2, 1.0, 1.0)] {
        let mut input = valid_input();
        input["speed"] = json!(speed);
        input["stability"] = json!(stability);
        input["similarity"] = json!(similarity);
        wav(input).await;
    }
}

#[tokio::test]
async fn malformed_or_incompatible_json_is_rejected() {
    for body in ["{", "null", "[]", "{}", r#"{"text": "abc", "speed": NaN}"#] {
        expect_error(body.into(), StatusCode::BAD_REQUEST, "invalid_json").await;
    }
    for field in ["text", "voice_id", "speed", "stability", "similarity"] {
        let mut input = valid_input();
        input.as_object_mut().unwrap().remove(field);
        expect_error(input.to_string(), StatusCode::BAD_REQUEST, "invalid_json").await;
        let mut input = valid_input();
        input[field] = json!({"wrong": "type"});
        expect_error(input.to_string(), StatusCode::BAD_REQUEST, "invalid_json").await;
    }
    let mut input = valid_input();
    input["extra"] = json!(true);
    expect_error(input.to_string(), StatusCode::BAD_REQUEST, "invalid_json").await;
}

#[tokio::test]
async fn wrong_content_type_is_rejected_as_json_error() {
    let response = post(valid_input().to_string(), "text/plain").await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
    let error: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(error["error"]["code"], "invalid_json");
}

#[tokio::test]
async fn oversized_body_is_rejected_before_generation() {
    let mut input = valid_input();
    input["text"] = json!("x".repeat(16 * 1024));
    expect_error(
        input.to_string(),
        StatusCode::PAYLOAD_TOO_LARGE,
        "request_too_large",
    )
    .await;
}
