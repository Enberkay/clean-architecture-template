use axum::{Json, http::StatusCode, response::IntoResponse};

pub async fn not_found() -> impl IntoResponse {
    let response = serde_json::json!({
        "success": false,
        "message": "Endpoint not found"
    });
    (StatusCode::NOT_FOUND, Json(response)).into_response()
}

pub async fn health_check() -> impl IntoResponse {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let response = serde_json::json!({
        "success": true,
        "message": "Service is healthy",
        "time": timestamp
    });
    (StatusCode::OK, Json(response)).into_response()
}
