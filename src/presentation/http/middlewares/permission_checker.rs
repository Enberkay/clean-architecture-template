use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower::{Layer, Service};

use crate::infrastructure::Claims;

#[derive(Clone)]
pub struct PermissionLayer {
    required_permissions: Vec<String>,
}

impl PermissionLayer {
    pub fn new(required_permissions: impl Into<Vec<String>>) -> Self {
        Self {
            required_permissions: required_permissions.into(),
        }
    }

    pub fn from_permissions(permissions: &[&str]) -> Self {
        Self {
            required_permissions: permissions.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl<S> Layer<S> for PermissionLayer {
    type Service = PermissionChecker<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PermissionChecker {
            inner,
            required_permissions: self.required_permissions.clone(),
        }
    }
}

#[derive(Clone)]
pub struct PermissionChecker<S> {
    inner: S,
    required_permissions: Vec<String>,
}

impl<S> Service<Request> for PermissionChecker<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let required_permissions = std::mem::take(&mut self.required_permissions);
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract JWT claims from request extensions
            let claims = match req.extensions().get::<Claims>() {
                Some(claims) => claims,
                None => {
                    let error = PermissionError::NoClaims;
                    return Ok(error.into_response());
                }
            };

            // Check if user has all required permissions
            let has_all_permissions = required_permissions
                .iter()
                .all(|required_permission| claims.permissions.contains(required_permission));

            if has_all_permissions {
                // User has all required permissions, continue with request
                inner.call(req).await
            } else {
                // User doesn't have required permissions
                let missing_permissions: Vec<String> = required_permissions
                    .iter()
                    .filter(|perm| !claims.permissions.contains(perm))
                    .cloned()
                    .collect();

                let error = PermissionError::InsufficientPermissions {
                    missing: missing_permissions,
                    user_permissions: claims.permissions.clone(),
                };
                Ok(error.into_response())
            }
        })
    }
}

#[derive(Debug)]
enum PermissionError {
    NoClaims,
    InsufficientPermissions {
        missing: Vec<String>,
        user_permissions: Vec<String>,
    },
}

impl std::fmt::Display for PermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionError::NoClaims => write!(f, "No authentication claims found"),
            PermissionError::InsufficientPermissions {
                missing,
                user_permissions,
            } => {
                write!(
                    f,
                    "Insufficient permissions. Missing: {:?}, User permissions: {:?}",
                    missing, user_permissions
                )
            }
        }
    }
}

impl IntoResponse for PermissionError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            PermissionError::NoClaims => (StatusCode::UNAUTHORIZED, "No authentication found"),
            PermissionError::InsufficientPermissions { .. } => (
                StatusCode::FORBIDDEN,
                "Access denied: Insufficient permissions",
            ),
        };

        let error_response = serde_json::json!({
            "success": false,
            "message": message,
            "error": self.to_string()
        });

        (status, Json(error_response)).into_response()
    }
}
