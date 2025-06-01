pub mod context;
use axum::{extract::Request, middleware::Next, response::Response};
use context::UserContext;
use worker::console_log;

static SKIP_PATHS: &[&str] = &[
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

    if SKIP_PATHS.contains(&path) {
        console_log!("Skipping token verification for path: {}", path);
        return next.run(req).await;
    }
    console_log!("Performing token verification for path: {}", path);

    req.headers_mut().get("Authorization").map(|auth_header| {
        console_log!("Authorization header found: {:?}", auth_header);
    });

    // tokenを検証するロジックをここに追加
    let user = UserContext::new("example_user_id".to_string());

    req.extensions_mut().insert(user);
    next.run(req).await
}
