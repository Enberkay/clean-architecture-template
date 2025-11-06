use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Common error type across the application (service/API) layer.
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl ApplicationError {
    /// Error for bad requests (validation, etc.)
    pub fn bad_request<T: Into<String>>(msg: T) -> Self {
        Self::BadRequest(msg.into())
    }

    /// Error for unauthorized access
    pub fn unauthorized<T: Into<String>>(msg: T) -> Self {
        Self::Unauthorized(msg.into())
    }

    /// Error for forbidden access
    pub fn forbidden<T: Into<String>>(msg: T) -> Self {
        Self::Forbidden(msg.into())
    }

    /// Error for missing resources
    pub fn not_found<T: Into<String>>(msg: T) -> Self {
        Self::NotFound(msg.into())
    }

    /// Error for conflicts (duplicate, etc.)
    pub fn conflict<T: Into<String>>(msg: T) -> Self {
        Self::Conflict(msg.into())
    }

    /// Error for internal server issues
    pub fn internal<T: Into<String>>(msg: T) -> Self {
        Self::Internal(msg.into())
    }
}

/// Map ApplicationError → JSON HTTP Response with proper status codes
impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            ApplicationError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            ApplicationError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            ApplicationError::Forbidden(_) => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            ApplicationError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            ApplicationError::Conflict(_) => (StatusCode::CONFLICT, "CONFLICT"),
            ApplicationError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        };

        let body = json!({
            "success": false,
            "error": self.to_string(),
            "code": code,
        });

        (status, Json(body)).into_response()
    }
}

/// Shortcut alias for application-level Result
pub type ApplicationResult<T> = Result<T, ApplicationError>;
