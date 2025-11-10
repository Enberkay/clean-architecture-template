use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

use crate::infrastructure::Claims;

/// Role-based Authorization Layer
/// Layer for verifying if user has required roles
#[derive(Clone)]
pub struct RoleLayer {
    required_roles: Vec<String>,
}

impl RoleLayer {
    /// Create a new role layer
    pub fn new(required_roles: impl Into<Vec<String>>) -> Self {
        Self {
            required_roles: required_roles.into(),
        }
    }

    /// Create from string slice array (more idiomatic)
    pub fn from_roles(roles: &[&str]) -> Self {
        Self {
            required_roles: roles.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl<S> Layer<S> for RoleLayer {
    type Service = RoleChecker<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RoleChecker {
            inner,
            required_roles: self.required_roles.clone(),
        }
    }
}

/// Role-based Authorization Checker Service
#[derive(Clone)]
pub struct RoleChecker<S> {
    inner: S,
    required_roles: Vec<String>,
}

impl<S> Service<Request> for RoleChecker<S>
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
        let required_roles = std::mem::take(&mut self.required_roles);
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract JWT claims from request extensions
            let claims = match req.extensions().get::<Claims>() {
                Some(claims) => claims,
                None => {
                    let error = RoleError::NoClaims;
                    return Ok(error.into_response());
                }
            };

            // Check if user has any of the required roles
            let has_required_role = required_roles
                .iter()
                .any(|required_role| claims.roles.contains(required_role));

            if has_required_role {
                // User has required role, continue with request
                inner.call(req).await
            } else {
                // User doesn't have required role
                let error = RoleError::InsufficientRoles {
                    required: required_roles,
                    user_roles: claims.roles.clone(),
                };
                Ok(error.into_response())
            }
        })
    }
}

/// Role authorization errors
#[derive(Debug)]
enum RoleError {
    NoClaims,
    InsufficientRoles {
        required: Vec<String>,
        user_roles: Vec<String>,
    },
}

impl std::fmt::Display for RoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleError::NoClaims => write!(f, "No authentication claims found"),
            RoleError::InsufficientRoles {
                required,
                user_roles,
            } => {
                write!(
                    f,
                    "Insufficient role permissions. Required: {:?}, User roles: {:?}",
                    required, user_roles
                )
            }
        }
    }
}

impl IntoResponse for RoleError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            RoleError::NoClaims => (StatusCode::UNAUTHORIZED, "No authentication found"),
            RoleError::InsufficientRoles { .. } => (
                StatusCode::FORBIDDEN,
                "Access denied: Insufficient role permissions",
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
