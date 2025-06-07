use crate::{
    api::response::create_octet_stream_response, internal::middleware::context::UserContext,
};
use axum::{body::Body, extract::Query, response::Response};
use chrono::NaiveDate;

use proto::generated::api::memo::get_data::{Memo, MemoList, QueryParameters};
use std::ops::Add;
use worker::console_log;

#[worker::send]
pub async fn get_from_date_handler(
    Query(query): Query<QueryParameters>,
    user: UserContext,
) -> Result<Response, Response> {
    let date = &query.date;

    let date = is_validate_date(date)
        .map_err(|_| create_invalid_date_error_response())
        .unwrap();
    console_log!(
        "get_data_from_date_handler invoked by user ID: {} with date: {}",
        user.user_id(),
        date.to_string()
    );

    let mock_data = MemoList {
        data: vec![
            Memo {
                date: date.to_string(),
                id: "dsadwadwadwarfs32".to_string(),
                short_contents: "short1...".to_string(),
            },
            Memo {
                date: date.add(chrono::Duration::days(1)).to_string(),
                id: "54534523f535".to_string(),
                short_contents: "short2...".to_string(),
            },
            Memo {
                date: date.add(chrono::Duration::days(2)).to_string(),
                id: "odfheuiawosrfy8e9iasrfh".to_string(),
                short_contents: "short3...".to_string(),
            },
        ],
    };

    let response = create_octet_stream_response(mock_data);

    let response = match response {
        Ok(resp) => resp,
        Err(e) => return Err(e),
    };

    Ok(response)
}

fn is_validate_date(date: &str) -> Result<NaiveDate, String> {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| "Invalid date format".to_string())?;

    Ok(date)
}

fn create_invalid_date_error_response() -> Response<Body> {
    let error_message = "Invalid date format. Please use YYYY-MM-DD.";
    let response = Response::builder()
        .status(400)
        .body(Body::from(error_message))
        .unwrap();
    response
}
