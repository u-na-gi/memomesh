use crate::{
    api::response::create_octet_stream_response, internal::middleware::context::UserContext,
};

use axum::{body::Body, extract::Query, response::Response};
use chrono::{Duration, NaiveDate};

use proto::generated::api::memo::get_count_by_date::{
    CountNotesByDate, CountNotesByDateList, QueryParameters,
};
use worker::console_log;

#[worker::send]
pub async fn get_count_by_date_handler(
    Query(query): Query<QueryParameters>,
    user: UserContext,
) -> Result<Response, Response> {
    console_log!("get_count_by_date_handler invoked");
    console_log!("User ID: {}", user.user_id());
    console_log!("Query parameters: {:?}", query);
    let from_date = &query.from_date;
    let to_date = &query.to_date;

    let (from_date, to_date) = is_validate_date_range(from_date, to_date)
        .map_err(|_| create_invalid_date_range_error_response())
        .unwrap();

    console_log!("Valid date range: from {} to {}", from_date, to_date);
    // 日付範囲に従ってデータを取得する処理を実装
    // ここではダミーのレスポンスを返す

    // モックデータ
    let mock_data: CountNotesByDateList = CountNotesByDateList {
        count_notes_by_date: vec![
            CountNotesByDate {
                date: "2023-10-01".to_string(),
                count: 5,
            },
            CountNotesByDate {
                date: "2023-10-02".to_string(),
                count: 3,
            },
            CountNotesByDate {
                date: "2023-10-03".to_string(),
                count: 8,
            },
            CountNotesByDate {
                date: "2023-10-04".to_string(),
                count: 5,
            },
        ],
    };

    let response = create_octet_stream_response(mock_data).map_err(|e| e)?;

    // Placeholder for the actual implementation
    Ok(response)
}

fn is_validate_date_range(
    from_date: &str,
    to_date: &str,
) -> Result<(NaiveDate, NaiveDate), String> {
    let from_date = NaiveDate::parse_from_str(from_date, "%Y-%m-%d")
        .map_err(|_| "Invalid from_date format".to_string())?;
    let to_date = NaiveDate::parse_from_str(to_date, "%Y-%m-%d")
        .map_err(|_| "Invalid to_date format".to_string())?;

    if from_date > to_date {
        return Err("from_date cannot be after to_date".to_string());
    }

    let max_range = Duration::days(31); // 1ヶ月として31日許容
    let actual_range = to_date - from_date;
    if actual_range > max_range {
        return Err("Date range exceeds 31 days".to_string());
    }

    Ok((from_date, to_date))
}

fn create_invalid_date_range_error_response() -> Response<Body> {
    let error_message = "Invalid date range. Please ensure that 'from_date' is before 'to_date' and the range does not exceed 31 days.";
    let response = Response::builder()
        .status(400)
        .body(Body::from(error_message))
        .unwrap();
    response
}
