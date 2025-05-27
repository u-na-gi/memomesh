use axum::{body::Body, http::StatusCode, response::Response};
use prost::Message;

use super::error::create_internal_server_error_response;

pub fn create_octet_stream_response<T: Message>(data: T) -> Result<Response, Response> {
    let mut buf = Vec::new();
    if let Err(_) = data.encode(&mut buf) {
        return Err(create_internal_server_error_response());
    }

    let response_body = Body::from(buf);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/octet-stream")
        .body(response_body)
        .unwrap();
    Ok(response)
}
