use axum::{http::StatusCode, response::IntoResponse};

pub enum AppError {
    UnHandledError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            Self::UnHandledError(err) => format!("something went wrong: {}", err),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(value: surrealdb::Error) -> Self {
        AppError::UnHandledError(value.to_string())
    }
}
