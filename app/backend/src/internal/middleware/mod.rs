pub mod context;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::{IntoResponse, Response}};
use context::UserContext;
use worker::console_log;

static TOKEN_VERIFY_SKIP_PATHS: &[&str] = &[
    "/api/v1/auth/login",
    "/api/v1/auth/session",
    "/api/v1/auth/forgot-password",
    "/api/v1/user/register",
];

pub async fn token_verify_handler(mut req: Request, next: Next) -> Response {
    // Placeholder for the actual middleware logic
    console_log!("Middleware handler invoked");

    let path = req.uri().path();
    console_log!("Request path: {}", path);

    if TOKEN_VERIFY_SKIP_PATHS.contains(&path) {
        console_log!("Skipping token verification for path: {}", path);
        return next.run(req).await;
    }
    console_log!("Performing token verification for path: {}", path);

    let auth_header = req.headers().get("Authorization")
    .ok_or((StatusCode::UNAUTHORIZED, "Authorization header not found").into_response())
    .and_then(|header| {
        header.to_str().map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid Authorization header").into_response())
    });

    let auth_header = match auth_header {
        Ok(header) => header.to_string(),
        Err(err) => return err,
    };

    console_log!("Authorization header: {}", auth_header);

    // tokenを検証するロジックをここに追加
    let user = UserContext::new("example_user_id".to_string());

    req.extensions_mut().insert(user);
    next.run(req).await
}


// session middlewareが必要じゃない??