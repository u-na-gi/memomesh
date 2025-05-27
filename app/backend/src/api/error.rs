use axum::{body::Body, response::Response};

pub fn create_internal_server_error_response() -> Response<Body> {
    let error_message = "An internal server error occurred while processing your request.";
    let response = Response::builder()
        .status(400)
        .body(Body::from(error_message))
        .unwrap();
    response
}

pub fn create_no_session_error_response() -> Response {
    let error_message = "No session found. Please log in.";
    let response = Response::builder()
        .status(404)
        .body(Body::from(error_message))
        .unwrap();
    response
}
