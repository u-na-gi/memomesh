use axum::{body::Bytes, response::Response};

use prost::Message;
use proto::generated::api::memo::edit::{EditMemoRequest, EditMemoResponse};
use worker::console_log;

use crate::{
    api::response::create_octet_stream_response, internal::middleware::context::UserContext,
};

#[worker::send]
pub async fn edit_memo_handler(
    user_context: UserContext,
    payload: Bytes,
) -> Result<Response, Response> {
    console_log!("edit_memo_handler invoked with payload: {:?}", payload);
    let payload = EditMemoRequest::decode(payload.as_ref()).map_err(|_| {
        console_log!("Failed to decode EditMemoRequest");
        Response::builder()
            .status(axum::http::StatusCode::BAD_REQUEST)
            .body(axum::body::Body::from("Invalid request payload"))
            .unwrap()
    })?;
    console_log!(
        "edit_memo_handler invoked by user ID: {} with payload: {:?}",
        &user_context.user_id(),
        payload
    );

    // TODO: データベースでメモを更新する処理を実装
    // 1. memo_idでメモを検索
    // 2. user_idで所有者確認
    // 3. contentとupdated_atを更新

    let response = EditMemoResponse {
        success: true,
        message: "メモが正常に更新されました".to_string(),
    };
    let response = create_octet_stream_response(response);

    let response = match response {
        Ok(resp) => resp,
        Err(e) => return Err(e),
    };
    Ok(response)
}
