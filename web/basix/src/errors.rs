use axum::{ http::StatusCode, response::{ IntoResponse, Response } };

pub type AppResult<T> = core::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Unknown,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = match &self {
            AppError::Unknown => "An unknown error occurred.",
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
