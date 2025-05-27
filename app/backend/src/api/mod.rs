pub mod auth;
pub mod error;
pub mod memo;
pub mod response;
pub mod user;
pub fn set_time_zone_handler() -> &'static str {
    "Set time zone handler"
}

pub fn health_check_handler() -> &'static str {
    "Health check handler"
}
