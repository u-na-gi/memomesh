use axum::{http::StatusCode, response::Response};

#[derive(Debug)]
pub enum EditMemoError {
    MemoNotFound,
    Unauthorized,
    DatabaseError(String),
    InvalidRequest(String),
}

impl EditMemoError {
    pub fn to_response(&self) -> Response {
        let (status, message) = match self {
            EditMemoError::MemoNotFound => (StatusCode::NOT_FOUND, "メモが見つかりません"),
            EditMemoError::Unauthorized => {
                (StatusCode::FORBIDDEN, "このメモを編集する権限がありません")
            }
            EditMemoError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str()),
            EditMemoError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
        };

        Response::builder()
            .status(status)
            .body(axum::body::Body::from(message))
            .unwrap()
    }
}
