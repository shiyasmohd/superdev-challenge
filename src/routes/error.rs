use axum::response::{IntoResponse, Response};

pub struct ErrorResponse {
    success: bool,
    error: String,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        Self {
            success: false,
            error,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "success": self.success,
            "error": self.error,
        });
        (axum::http::StatusCode::BAD_REQUEST, axum::Json(body)).into_response()
    }
}
