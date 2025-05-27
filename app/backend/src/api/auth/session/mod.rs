use axum::{body::Body, http::StatusCode, response::Response};
use axum_extra::extract::CookieJar;
use worker::console_log;

use crate::api::error::create_no_session_error_response;

#[worker::send]
pub async fn session_handler(jar: CookieJar) -> Result<Response, Response> {
    let session_value: Option<String> = jar
        .get("session_id")
        .map(|cookie| cookie.value().to_owned());
    let session_value = match session_value {
        Some(value) => value,
        None => return Err(create_no_session_error_response()),
    };

    // sessionの検証
    console_log!("Session ID: {}", session_value);

    let msg = "Session is valid.";
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(msg))
        .unwrap();
    Ok(response)
}

// #[worker::send]
// pub async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<Response, Infallible> {
//     // ユーザー名とパスワードを受け取る
//     let username: &String = &payload.username.into();
//     let password: &String = &payload.password.into();
//     console_log!("username: {}", username);
//     console_log!("password: {}", password);

//     // ユーザーが存在するか確認（成功ケースのみ実装）
//     // 実際はDBを参照するが、ここでは常に成功するとする

//     // パスワード照合（適当な値を返す）
//     // 実際はハッシュ化されたパスワードと比較するが、ここでは常に成功するとする

//     // JWT発行（有効期限5分）
//     let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

//     // セッションID生成（30日）
//     let session_id = "session_12345";

//     // リフレッシュトークン生成（30日）
//     let refresh_token = "refresh_token_12345";

//     // 開発ビルドとリリースビルドで切り替え
//     let is_production = cfg!(not(debug_assertions)); // リリースビルド時はtrue、開発ビルド時はfalse

//     // セッションIDクッキー設定
//     let session_cookie = Cookie::build(("session_id", session_id.to_string()))
//         .http_only(true)
//         .secure(is_production) // リリースビルドではSecureを有効
//         .path("/")
//         .max_age(Duration::seconds(30 * 24 * 60 * 60)); // 30日

//     // リフレッシュトークンクッキー設定
//     let refresh_cookie = Cookie::build(("refresh_token", refresh_token.to_string()))
//         .http_only(true)
//         .secure(is_production) // リリースビルドではSecureを有効
//         .path("/")
//         .max_age(Duration::seconds(30 * 24 * 60 * 60)); // 30日

//     // レスポンス作成
//     let response_body = LoginResponse {
//         jwt: jwt.to_string(),
//     };

//     let response_body = Body::from(serde_json::to_string(&response_body).unwrap());

//     let mut response = (StatusCode::OK, response_body).into_response();
//     for cookie in &[session_cookie, refresh_cookie] {
//         response.headers_mut().append(
//             SET_COOKIE,
//             HeaderValue::from_str(&cookie.to_string()).unwrap(),
//         );
//     }

//     Ok(response)
// }
