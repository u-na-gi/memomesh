use std::convert::Infallible;

use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::HeaderValue;
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Response, Json};
use axum_extra::extract::cookie::Cookie;
use chrono::{Duration, Utc};
use jwt_compact::alg::{Hs256, Hs256Key};

use jwt_compact::AlgorithmExt;
use jwt_compact::{Claims, Header};
use serde::{Deserialize, Serialize};
use time::Duration as TimeDuration;
// use ::{encode, EncodingKey, Header};

// protobufで生成された型をインポート
use proto::generated::api::auth::login::{LoginRequest, LoginResponse};
use worker::console_log;

#[derive(Debug, Serialize, Deserialize)]
struct CustomClaims {
    #[serde(rename = "exp")]
    exp: usize, // 有効期限（UNIXタイムスタンプ）
    #[serde(rename = "iat")]
    iat: usize, // 発行時刻（UNIXタイムスタンプ）
    #[serde(rename = "sub")]
    subject: String,
}

#[worker::send]
pub async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<Response, Infallible> {
    // ユーザー名とパスワードを受け取る
    let username: &String = &payload.username.into();
    let password: &String = &payload.password.into();
    console_log!("username: {}", username);
    console_log!("password: {}", password);

    // ユーザーが存在するか確認（成功ケースのみ実装）
    // 実際はDBを参照するが、ここでは常に成功するとする

    // パスワード照合（適当な値を返す）
    // 実際はハッシュ化されたパスワードと比較するが、ここでは常に成功するとする

    // JWT発行（有効期限5分）
    let jwt = create_jwt("user-1234534").unwrap(); // ここではダミーのJWTを使用
                                                   // セッションID生成（30日）
    let session_id = "session_12345";

    // リフレッシュトークン生成（30日）
    let refresh_token = "refresh_token_12345";

    // 開発ビルドとリリースビルドで切り替え
    let is_production = cfg!(not(debug_assertions)); // リリースビルド時はtrue、開発ビルド時はfalse

    // セッションIDクッキー設定
    let session_cookie = Cookie::build(("session_id", session_id.to_string()))
        .http_only(true)
        .secure(is_production) // リリースビルドではSecureを有効
        .path("/api/v1/auth")
        .max_age(TimeDuration::seconds(30 * 24 * 60 * 60)); // 30日

    // リフレッシュトークンクッキー設定
    let refresh_cookie = Cookie::build(("refresh_token", refresh_token.to_string()))
        .http_only(true)
        .secure(is_production) // リリースビルドではSecureを有効
        .path("/api/v1/auth/refresh")
        .max_age(TimeDuration::seconds(30 * 24 * 60 * 60)); // 30日

    // レスポンス作成
    let response_body = LoginResponse { jwt: jwt };

    let response_body = Body::from(serde_json::to_string(&response_body).unwrap());

    let mut response = (StatusCode::OK, response_body).into_response();
    for cookie in &[session_cookie, refresh_cookie] {
        response.headers_mut().append(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    Ok(response)
}

/// 秘密鍵（HMAC）で署名された JWT を発行する
pub fn create_jwt(subject: &str) -> Result<String, anyhow::Error> {
    let key = Hs256Key::new(b"super_secret_key_donut_steel");

    let header = Header::empty();
    let claims = Claims::new(CustomClaims {
        exp: (Utc::now() + Duration::days(30)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
        subject: subject.to_string(),
    });

    let token = Hs256.token(&header, &claims, &key)?;
    Ok(token)
}
