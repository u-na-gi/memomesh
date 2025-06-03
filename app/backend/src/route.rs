use axum::{
    middleware::from_fn,
    routing::{delete, get, post},
    Router,
};

use crate::{
    api::{
        auth::{
            forgot_password::forgot_password_handler, login::login_handler, logout::logout_handler,
            session::session_handler,
        },
        memo::get::{by_id::get_memo_by_id_handler, from_date::get_from_date_handler},
    },
    internal::middleware::token_verify_handler,
};
use crate::{
    api::{
        memo::{
            create::create_memo_handler, delete::delete_memo_handler, edit::edit_memo_handler,
            get_count_by_date::get_count_by_date_handler,
        },
        user::register::register_handler,
    },
    internal::components::cors::create_cors_layer,
};

pub fn router() -> Router {
    // ログインを作ろう

    let router = Router::new()
        .route("/", get(root))
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/auth/logout", delete(logout_handler))
        .route("/api/v1/auth/session", get(session_handler))
        .route(
            "/api/v1/auth/forgot-password",
            post(forgot_password_handler),
        )
        .route(
            "/api/v1/memo/count-notes-by-date",
            get(get_count_by_date_handler),
        )
        .route("/api/v1/memo/data", get(get_from_date_handler))
        .route("/api/v1/memo/id", get(get_memo_by_id_handler))
        .route("/api/v1/memo/create", post(create_memo_handler))
        .route("/api/v1/memo/delete", delete(delete_memo_handler))
        .route("/api/v1/memo/edit", post(edit_memo_handler))
        .route("/api/v1/user/register", post(register_handler))
        .layer(from_fn(token_verify_handler))
        .layer(create_cors_layer());

    router
}

async fn root() -> &'static str {
    "Hello Axum!"
}
