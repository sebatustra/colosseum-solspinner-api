use axum::{extract::Request, http::HeaderMap, middleware::Next, response::{IntoResponse, Response}, Extension};
use reqwest::StatusCode;
use crate::AppState;

pub async fn auth_middleware(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    println!("->> {:<12} - auth_middleware", "MIDDLEWARE");

    let api_key = &state.api_key;

    match headers.get("authorization") {
        Some(header_value) => {
            match header_value.to_str() {
                Ok(auth_str) => {
                    if api_key == auth_str {
                        next.run(request).await
                    } else {
                        StatusCode::UNAUTHORIZED.into_response()
                    }
                },
                Err(_) => StatusCode::UNAUTHORIZED.into_response()
            }
        },
        None => StatusCode::UNAUTHORIZED.into_response()
    }
}