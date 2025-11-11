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
use tracing::debug;

use crate::{
    infrastructure::validate_access_token_claims,
    presentation::http::cookie_utils::extract_access_token_from_cookie,
    application::dtos::auth_dto::UserInfo,
};

#[derive(Clone)]
pub struct JwtAuthLayer {
    jwt_secret: String,
}

impl JwtAuthLayer {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl<S> Layer<S> for JwtAuthLayer {
    type Service = JwtAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        JwtAuthMiddleware {
            inner,
            jwt_secret: self.jwt_secret.clone(),
        }
    }
}

#[derive(Clone)]
pub struct JwtAuthMiddleware<S> {
    inner: S,
    jwt_secret: String,
}

impl<S> Service<Request> for JwtAuthMiddleware<S>
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

    fn call(&mut self, mut req: Request) -> Self::Future {
        let jwt_secret = self.jwt_secret.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract access token from HttpOnly cookie (แทน Authorization header)
            let access_token = extract_access_token_from_cookie(req.headers());

            match access_token {
                Some(token) => {
                    debug!("Found access token in cookie, validating...");
                    
                    // Validate JWT token
                    match validate_access_token_claims(&token, &jwt_secret) {
                        Ok(claims) => {
                            debug!("Access token valid for user: {}", claims.sub);
                            
                            // Insert claims into request extensions for downstream handlers
                            req.extensions_mut().insert(claims);
                            
                            // Continue with the request
                            inner.call(req).await
                        }
                        Err(e) => {
                            debug!("Invalid access token: {}", e);
                            // Invalid token
                            let error = AuthError::InvalidToken;
                            Ok(error.into_response())
                        }
                    }
                }
                None => {
                    debug!("No access token found in cookie");
                    // Missing access token
                    let error = AuthError::MissingAccessToken;
                    Ok(error.into_response())
                }
            }
        })
    }
}



#[derive(Debug)]
pub enum AuthError {
    MissingAccessToken,
    InvalidToken,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingAccessToken => write!(f, "Missing access token"),
            AuthError::InvalidToken => write!(f, "Invalid or expired access token"),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingAccessToken => {
                (StatusCode::UNAUTHORIZED, "Missing access token")
            }
            AuthError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid or expired access token")
            }
        };

        let error_response = serde_json::json!({
            "success": false,
            "message": message,
            "error": self.to_string()
        });

        (status, Json(error_response)).into_response()
    }
}

/// Helper function to extract user info from request extensions
pub fn extract_user_info(req: &Request) -> Result<UserInfo, AuthError> {
    let claims = req.extensions().get::<crate::infrastructure::Claims>()
        .ok_or(AuthError::MissingAccessToken)?;

    // Mock user info - ใน production ควร query จาก database
    let user_info = UserInfo {
        id: claims.sub.parse().unwrap_or(0),
        email: "user@example.com".to_string(), // ควรดึงจาก DB
        fname: "User".to_string(),             // ควรดึงจาก DB
        lname: "Name".to_string(),             // ควรดึงจาก DB
        roles: claims.roles.clone(),
        permissions: claims.permissions.clone(),
    };

    Ok(user_info)
}