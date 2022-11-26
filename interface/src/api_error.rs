use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::json;

pub struct ApiError {
    pub status: StatusCode,
    pub description: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "api_error": {
                "status": self.status.to_string(),
                "description": self.description,
            }
        }));

        (self.status, body).into_response()
    }
}
