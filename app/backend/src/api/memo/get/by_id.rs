use axum::{extract::Query, response::Response};
use proto::generated::api::memo::get_by_id::{GetMemoByIdResponse, QueryParameters};
use worker::console_log;

use crate::{
    api::response::create_octet_stream_response, internal::middleware::context::UserContext,
};

#[worker::send]
pub async fn get_memo_by_id_handler(
    user_context: UserContext,
    Query(params): Query<QueryParameters>,
) -> Result<Response, Response> {
    console_log!("get_memo_by_id_handler invoked with id: {}", params.id);
    console_log!(
        "get_memo_by_id_handler invoked by user ID: {} with id: {}",
        &user_context.user_id(),
        params.id
    );

    // TODO: 実際のデータベースからメモを取得する処理を実装
    let response = GetMemoByIdResponse {
        id: params.id,
        date: "2025-06-04".to_string(),
        contents: "Sample memo contents".to_string(),
        parent_id: "sample_parent_id".to_string(),
        created_at: "2025-06-04T02:00:00Z".to_string(),
        updated_at: "2025-06-04T02:00:00Z".to_string(),
    };

    let response = create_octet_stream_response(response);

    let response = match response {
        Ok(resp) => resp,
        Err(e) => return Err(e),
    };
    Ok(response)
}
