use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserContext {
    pub user_id: String,
}

impl UserContext {
    pub fn new(user_id: String) -> Self {
        UserContext { user_id }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserContext
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // axum の Extension extractor を使って extensions から取り出す
        if let Some(user) = parts.extensions.get::<UserContext>() {
            Ok(user.clone()) // Clone が必要
        } else {
            Err((StatusCode::UNAUTHORIZED, "UserContext not found").into_response())
        }
    }
}
