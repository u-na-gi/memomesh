use axum::{body::Bytes, response::Response};

use prost::Message;
use proto::generated::api::memo::delete::{DeleteMemoRequest, DeleteMemoResponse};
use worker::console_log;

use crate::{
    api::response::create_octet_stream_response, internal::middleware::context::UserContext,
};

#[worker::send]
pub async fn delete_memo_handler(
    user_context: UserContext,
    payload: Bytes,
) -> Result<Response, Response> {
    console_log!("delete_memo_handler invoked with payload: {:?}", payload);
    let payload = DeleteMemoRequest::decode(payload.as_ref()).map_err(|_| {
        console_log!("Failed to decode DeleteMemoRequest");
        Response::builder()
            .status(axum::http::StatusCode::BAD_REQUEST)
            .body(axum::body::Body::from("Invalid request payload"))
            .unwrap()
    })?;
    console_log!(
        "delete_memo_handler invoked by user ID: {} with payload: {:?}",
        &user_context.user_id(),
        payload
    );

    // TODO: データベースでメモを削除する処理を実装
    // 1. memo_idでメモを検索
    // 2. user_idで所有者確認
    // 3. メモを削除

    let response = DeleteMemoResponse {
        success: true,
        message: "メモが正常に削除されました".to_string(),
    };
    let response = create_octet_stream_response(response);

    let response = match response {
        Ok(resp) => resp,
        Err(e) => return Err(e),
    };
    Ok(response)
}
