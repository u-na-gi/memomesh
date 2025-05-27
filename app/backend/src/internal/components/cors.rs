use axum::http::{HeaderName, HeaderValue};
use tower_http::cors::{AllowOrigin, CorsLayer};
use worker::console_log;

const ALLOWED_SUBSTRINGS: [&str; 1] = ["http://localhost:3333"];
const ALLOWED_HEADERS: [HeaderName; 4] = [
    axum::http::header::AUTHORIZATION,
    axum::http::header::CONTENT_TYPE,
    axum::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
    axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
];

fn custom_allow_origin(origin: &HeaderValue) -> bool {
    let origin_str = origin.to_str().unwrap_or("");
    ALLOWED_SUBSTRINGS
        .iter()
        .any(|&substring| origin_str.contains(substring))
}

pub fn create_cors_layer() -> CorsLayer {
    // 開発ビルドとリリースビルドで切り替え
    let is_production = cfg!(not(debug_assertions)); // リリースビルド時はtrue、開発ビルド時はfalse

    let cors = if is_production {
        console_log!("本番環境: 特定のオリジンのみ許可");
        // 本番環境: 特定のオリジンのみ許可
        CorsLayer::new()
            .allow_origin(AllowOrigin::predicate(|origin, _| {
                let allowed_substrings = ["unagi-id-bwb"];
                let origin_str = origin.to_str().unwrap_or("");
                allowed_substrings
                    .iter()
                    .any(|&substring| origin_str.contains(substring))
            }))
            .allow_headers(ALLOWED_HEADERS)
    } else {
        console_log!("開発環境: すべてのオリジンを許可");
        // 開発環境: すべてのオリジンを許可
        CorsLayer::new()
            .allow_origin(AllowOrigin::predicate(|origin, _| {
                custom_allow_origin(origin)
            }))
            .allow_headers(ALLOWED_HEADERS)
            .allow_credentials(true)
    };

    cors
}
