pub mod context;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use context::UserContext;
use worker::console_log;

static AUTH_SKIP_PATHS: &[&str] = &[
    "/api/v1/auth/login",
    "/api/v1/auth/forgot-password",
    "/api/v1/user/register",
];

static SESSION_INCLUDE_PATHS: &[&str] = &["/api/v1/auth/logout", "/api/v1/auth/session"];

pub async fn token_verify_handler(mut req: Request, next: Next) -> Response {
    // Placeholder for the actual middleware logic
    console_log!("Middleware handler invoked");

    let path = req.uri().path();
    console_log!("Request path: {}", path);

    if AUTH_SKIP_PATHS.contains(&path) || SESSION_INCLUDE_PATHS.contains(&path) {
        console_log!("Skipping token verification for path: {}", path);
        return next.run(req).await;
    }
    console_log!("Performing token verification for path: {}", path);

    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or((StatusCode::UNAUTHORIZED, "Authorization header not found").into_response())
        .and_then(|header| {
            header.to_str().map_err(|_| {
                (StatusCode::UNAUTHORIZED, "Invalid Authorization header").into_response()
            })
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

pub async fn session_verify_handler(mut req: Request, next: Next) -> Response {
    console_log!("Session middleware handler invoked");

    let path = req.uri().path();
    console_log!("Request path: {}", path);

    if AUTH_SKIP_PATHS.contains(&path) || !SESSION_INCLUDE_PATHS.contains(&path) {
        console_log!("Skipping session verification for path: {}", path);
        return next.run(req).await;
    }
    console_log!("Performing session verification for path: {}", path);

    // Cookieからセッション情報を取得
    let session_cookie = req.headers().get("Cookie").and_then(|cookie_header| {
        cookie_header.to_str().ok().and_then(|cookies| {
            // セッションCookieを探す（例: "session=xxx"）
            cookies.split(';').find_map(|cookie| {
                let cookie = cookie.trim();
                if cookie.starts_with("session_id=") {
                    Some(cookie.strip_prefix("session_id=").unwrap_or(""))
                } else {
                    None
                }
            })
        })
    });

    let session_id = match session_cookie {
        Some(session) if !session.is_empty() => {
            console_log!("Session found: {}", session);
            session.to_string()
        }
        _ => {
            console_log!("Session not found in cookies");
            return (StatusCode::UNAUTHORIZED, "Session not found").into_response();
        }
    };

    console_log!("Session ID: {}", session_id);

    // セッション検証ロジックをここに追加（現在はプレースホルダー）
    let user = UserContext::new("example_user_id".to_string());

    req.extensions_mut().insert(user);
    next.run(req).await
}

// session middlewareが必要じゃない??
