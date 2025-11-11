use axum::{
    http::{header::SET_COOKIE, HeaderMap, HeaderValue},
    response::Response,
};

/// Cookie configuration constants
pub const ACCESS_TOKEN_NAME: &str = "accessToken";
pub const REFRESH_TOKEN_NAME: &str = "refreshToken";

/// Set HttpOnly access token cookie (15 minutes)
pub fn set_access_token_cookie(response: Response, token: &str) -> Response {
    let mut response = response;
    let cookie_value = format!(
        "{}={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age={}",
        ACCESS_TOKEN_NAME,
        token,
        15 * 60 // 15 minutes in seconds
    );

    if let Ok(header_value) = HeaderValue::from_str(&cookie_value) {
        response.headers_mut().insert(SET_COOKIE, header_value);
    }

    response
}

/// Set HttpOnly refresh token cookie (7 days, limited to auth endpoints)
pub fn set_refresh_token_cookie(response: Response, token: &str) -> Response {
    let mut response = response;
    let cookie_value = format!(
        "{}={}; HttpOnly; Secure; SameSite=Lax; Path=/api/auth; Max-Age={}",
        REFRESH_TOKEN_NAME,
        token,
        7 * 24 * 60 * 60 // 7 days in seconds
    );

    if let Ok(header_value) = HeaderValue::from_str(&cookie_value) {
        response.headers_mut().insert(SET_COOKIE, header_value);
    }

    response
}

/// Extract access token from cookie
pub fn extract_access_token_from_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get("cookie")?.to_str().ok()?;
    
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(token_part) = cookie.strip_prefix(&format!("{}=", ACCESS_TOKEN_NAME)) {
            return Some(token_part.to_string());
        }
    }
    
    None
}

/// Extract refresh token from cookie
pub fn extract_refresh_token_from_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get("cookie")?.to_str().ok()?;
    
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(token_part) = cookie.strip_prefix(&format!("{}=", REFRESH_TOKEN_NAME)) {
            return Some(token_part.to_string());
        }
    }
    
    None
}

/// Clear access token cookie
pub fn clear_access_token_cookie(mut response: Response) -> Response {
    let cookie_value = format!(
        "{}=; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
        ACCESS_TOKEN_NAME
    );

    if let Ok(header_value) = HeaderValue::from_str(&cookie_value) {
        response.headers_mut().insert(SET_COOKIE, header_value);
    }

    response
}

/// Clear refresh token cookie
pub fn clear_refresh_token_cookie(mut response: Response) -> Response {
    let cookie_value = format!(
        "{}=; HttpOnly; Secure; SameSite=Lax; Path=/api/auth; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
        REFRESH_TOKEN_NAME
    );

    if let Ok(header_value) = HeaderValue::from_str(&cookie_value) {
        response.headers_mut().insert(SET_COOKIE, header_value);
    }

    response
}

/// Clear both access and refresh token cookies
pub fn clear_all_auth_cookies(response: Response) -> Response {
    let response = clear_access_token_cookie(response);
    clear_refresh_token_cookie(response)
}