use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum ForgotPasswordError {
    InvalidEmail,
    InternalError,
}

impl IntoResponse for ForgotPasswordError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ForgotPasswordError::InvalidEmail => {
                (StatusCode::BAD_REQUEST, "無効なメールアドレスです")
            }
            ForgotPasswordError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "内部エラーが発生しました",
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
