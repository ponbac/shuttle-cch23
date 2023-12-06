use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: impl std::fmt::Display) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = format!("{}: {}", self.status, self.message);

        (self.status, body).into_response()
    }
}
