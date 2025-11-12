use axum::{
    http::{header::SET_COOKIE, HeaderMap, HeaderValue},
    response::Response,
};

// Config will be passed as parameters, not imported

/// Cookie configuration constants
pub const ACCESS_TOKEN_NAME: &str = "accessToken";
pub const REFRESH_TOKEN_NAME: &str = "refreshToken";

/// Set HttpOnly access token cookie (15 minutes)
pub fn set_access_token_cookie(mut response: Response, token: &str, max_age: i64) -> Response {
    // Validate token format (basic check)
    if token.is_empty() {
        tracing::warn!("Attempted to set empty access token cookie");
        return response;
    }

    let cookie_value = format!(
        "{}={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age={}",
        ACCESS_TOKEN_NAME,
        token,
        max_age
    );

    match HeaderValue::from_str(&cookie_value) {
        Ok(header_value) => {
            response.headers_mut().insert(SET_COOKIE, header_value);
        }
        Err(e) => {
            tracing::error!("Failed to create access token cookie header: {}", e);
        }
    }

    response
}

/// Set HttpOnly refresh token cookie (7 days, limited to auth endpoints)
pub fn set_refresh_token_cookie(mut response: Response, token: &str, max_age: i64) -> Response {
    // Validate token format (basic check)
    if token.is_empty() {
        tracing::warn!("Attempted to set empty refresh token cookie");
        return response;
    }

    let cookie_value = format!(
        "{}={}; HttpOnly; Secure; SameSite=Lax; Path=/api/auth; Max-Age={}",
        REFRESH_TOKEN_NAME,
        token,
        max_age
    );

    match HeaderValue::from_str(&cookie_value) {
        Ok(header_value) => {
            response.headers_mut().insert(SET_COOKIE, header_value);
        }
        Err(e) => {
            tracing::error!("Failed to create refresh token cookie header: {}", e);
        }
    }

    response
}

/// Extract access token from cookie
pub fn extract_access_token_from_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get("cookie")?.to_str().ok()?;
    
    cookie_header
        .split(';')
        .find_map(|cookie| {
            let cookie = cookie.trim();
            cookie.strip_prefix(&format!("{}=", ACCESS_TOKEN_NAME))
                .map(|token| token.to_string())
        })
}

/// Extract refresh token from cookie
pub fn extract_refresh_token_from_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get("cookie")?.to_str().ok()?;
    
    cookie_header
        .split(';')
        .find_map(|cookie| {
            let cookie = cookie.trim();
            cookie.strip_prefix(&format!("{}=", REFRESH_TOKEN_NAME))
                .map(|token| token.to_string())
        })
}

/// Clear access token cookie
pub fn clear_access_token_cookie(mut response: Response) -> Response {
    let cookie_value = format!(
        "{}=; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
        ACCESS_TOKEN_NAME
    );

    match HeaderValue::from_str(&cookie_value) {
        Ok(header_value) => {
            response.headers_mut().insert(SET_COOKIE, header_value);
        }
        Err(e) => {
            tracing::error!("Failed to create clear access token cookie header: {}", e);
        }
    }

    response
}

/// Clear refresh token cookie
pub fn clear_refresh_token_cookie(mut response: Response) -> Response {
    let cookie_value = format!(
        "{}=; HttpOnly; Secure; SameSite=Lax; Path=/api/auth; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
        REFRESH_TOKEN_NAME
    );

    match HeaderValue::from_str(&cookie_value) {
        Ok(header_value) => {
            response.headers_mut().insert(SET_COOKIE, header_value);
        }
        Err(e) => {
            tracing::error!("Failed to create clear refresh token cookie header: {}", e);
        }
    }

    response
}

/// Clear both access and refresh token cookies
pub fn clear_all_auth_cookies(mut response: Response) -> Response {
    response = clear_access_token_cookie(response);
    clear_refresh_token_cookie(response)
}