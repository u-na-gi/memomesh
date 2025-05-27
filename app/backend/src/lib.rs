pub mod api;
pub mod internal;
mod route;

use route::router;
use tower_service::Service;
use worker::*;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    // routerを定義する

    Ok(router().call(req).await?)
}
