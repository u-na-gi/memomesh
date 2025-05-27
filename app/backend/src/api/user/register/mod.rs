use axum::{body::Bytes, response::Response};
use prost::Message;
use proto::generated::api::user::register::{RegisterRequest, RegisterResponse};
use worker::console_log;

use crate::api::response::create_octet_stream_response;

#[worker::send]
pub async fn register_handler(
    payload: Bytes, // Assuming payload is a JSON object
) -> Result<Response, Response> {
    let payload = RegisterRequest::decode(payload.as_ref()).map_err(|_| {
        console_log!("Failed to decode RegisterRequest");
        Response::builder()
            .status(axum::http::StatusCode::BAD_REQUEST)
            .body(axum::body::Body::from("Invalid request payload"))
            .unwrap()
    })?;
    console_log!("Register handler invoked with payload: {:?}", payload);

    let response = RegisterResponse {
        message: "User registered successfully".to_string(),
    };
    create_octet_stream_response(response)
}
