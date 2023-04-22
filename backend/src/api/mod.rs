use std::sync::{mpsc, Arc};

use axum::{http::StatusCode, response::IntoResponse};
use tokio::sync::Mutex;

use crate::db::DbRequest;

#[derive(Clone)]
pub struct AppState {
    pub req_send: Arc<Mutex<mpsc::Sender<DbRequest>>>,
}

impl AppState {
    pub fn new(req_send: Arc<Mutex<mpsc::Sender<DbRequest>>>) -> Self {
        Self { req_send }
    }
}
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
