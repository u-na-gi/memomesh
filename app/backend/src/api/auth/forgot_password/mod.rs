use std::convert::Infallible;

use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Response, Json};
use proto::generated::api::auth::forgot_password::forgot_password::{
    ForgotPasswordRequest, ForgotPasswordResponse,
};
use worker::console_log;

pub mod error;

#[worker::send]
pub async fn forgot_password_handler(
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Response, Infallible> {
    // メールアドレスを受け取る
    let email: &String = &payload.email.into();
    console_log!("パスワードリセット要求: email: {}", email);

    // 簡単なメールアドレス形式チェック
    if !email.contains('@') || email.is_empty() {
        console_log!("無効なメールアドレス: {}", email);
        let response_body = ForgotPasswordResponse {
            message: "パスワードリセットの指示をメールで送信しました。".to_string(),
        };
        let response = (StatusCode::OK, Json(response_body)).into_response();
        return Ok(response);
    }

    // 実際の実装では以下を行う：
    // 1. データベースでユーザーの存在確認
    // 2. パスワードリセットトークンの生成
    // 3. メール送信
    //
    // セキュリティ上の理由で、メールアドレスが存在するかどうかに関わらず
    // 同じレスポンスを返す

    console_log!("パスワードリセット処理完了（実際のメール送信は未実装）");

    // レスポンス作成
    let response_body = ForgotPasswordResponse {
        message: "パスワードリセットの指示をメールで送信しました。".to_string(),
    };

    let response = (StatusCode::OK, Json(response_body)).into_response();
    Ok(response)
}
