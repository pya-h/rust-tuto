use axum::{ http::StatusCode, response::{ IntoResponse, Response } };

pub type AppResult<T> = core::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    LoginFailure,
    Unknown,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            match &self {
                AppError::LoginFailure => (StatusCode::UNAUTHORIZED, "Invalid credentials!"),
                AppError::Unknown =>
                    (StatusCode::INTERNAL_SERVER_ERROR, "An unknown error occurred."),
            }
        ).into_response()
    }
}
