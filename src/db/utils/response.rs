pub use http::Response;
use std::collections::HashMap;

// This type will be used to serialize the generic response body.
// Example: {"status": "ok"}
pub type ResponseBody = HashMap<&'static str, &'static str>;

pub fn create_response(code: u16, body: Option<ResponseBody>) -> Response<String> {
    let code = http::StatusCode::from_u16(code).unwrap();

    // Serialize the body if provided.
    let body = if let Some(body) = body {
        serde_json::to_string(&body).unwrap()
    } else {
        // Default to an empty object.
        "{}".to_string()
    };

    Response::builder().status(code).body(body).unwrap()
}

pub fn get_not_allowed_response() -> Response<String> {
    create_response(405, None)
}

pub fn get_not_found_response(body: Option<ResponseBody>) -> Response<String> {
    create_response(404, body)
}
