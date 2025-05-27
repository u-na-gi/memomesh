use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::HeaderValue;
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Response};
use axum_extra::extract::cookie::Cookie;

use axum_extra::extract::CookieJar;
use time::Duration;
use worker::console_log;

use crate::api::error::create_no_session_error_response;

#[worker::send]
pub async fn logout_handler(jar: CookieJar) -> Result<Response, Response> {
    console_log!("Logging out user");
    let session_value: Option<String> = jar
        .get("session_id")
        .map(|cookie| cookie.value().to_owned());
    let session_value = match session_value {
        Some(value) => value,
        None => return Err(create_no_session_error_response()),
    };

    // sessionの検証
    console_log!("Session ID: {}", session_value);

    // 開発ビルドとリリースビルドで切り替え
    let is_production = cfg!(not(debug_assertions)); // リリースビルド時はtrue、開発ビルド時はfalse

    // セッションIDクッキーを無効化
    let session_cookie = Cookie::build(("session_id", ""))
        .http_only(true)
        .secure(is_production)
        .path("/api/v1/auth")
        .max_age(Duration::seconds(0)); // 即時無効化

    // リフレッシュトークンクッキーを無効化
    let refresh_cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .secure(is_production)
        .path("/api/v1/auth/refresh")
        .max_age(Duration::seconds(0)); // 即時無効化

    // レスポンス作成
    let response_body = Body::from("Logged out successfully");
    let mut response = (StatusCode::OK, response_body).into_response();

    // Cookieヘッダーを設定
    for cookie in &[session_cookie, refresh_cookie] {
        response.headers_mut().append(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    Ok(response)
}
