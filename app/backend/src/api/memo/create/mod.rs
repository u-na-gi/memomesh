use axum::{body::Bytes, response::Response};

use prost::Message;
use proto::generated::api::memo::create::{CreateMemoRequest, CreateMemoResponse};
use worker::console_log;

use crate::{
    api::response::create_octet_stream_response, internal::middleware::context::UserContext,
};

#[worker::send]
pub async fn create_memo_handler(
    user_context: UserContext,
    payload: Bytes,
) -> Result<Response, Response> {
    console_log!("create_memo_handler invoked with payload: {:?}", payload);
    let payload = CreateMemoRequest::decode(payload.as_ref()).map_err(|_| {
        console_log!("Failed to decode CreateMemoRequest");
        Response::builder()
            .status(axum::http::StatusCode::BAD_REQUEST)
            .body(axum::body::Body::from("Invalid request payload"))
            .unwrap()
    })?;
    console_log!(
        "create_memo_handler invoked by user ID: {} with payload: {:?}",
        &user_context.user_id(),
        payload
    );
    let response = CreateMemoResponse {};
    let response = create_octet_stream_response(response);

    let response = match response {
        Ok(resp) => resp,
        Err(e) => return Err(e),
    };
    Ok(response)
}
