pub mod app_error;
pub mod app_state;
pub mod axum_app;

use std::sync::mpsc;

pub use app_error::AppError;
pub use app_state::AppState;
pub use axum_app::AxumApp;

use crate::db::{DbAction, DbRequest, DbResponse, DbResult};

pub async fn make_db_request(state: AppState, action: DbAction) -> Result<DbResult, AppError> {
    match state.req_send.lock().await.into() {
        Some(sender) => {
            println!("Lock acquired on Sender");
            let (responder, resp_receiver) = mpsc::channel::<DbResponse>();
            let sent_status = sender.send(DbRequest { action, responder });
            match sent_status {
                Ok(()) => {
                    println!("Request Sent Successfully!");
                    match resp_receiver.recv() {
                        Err(err) => Err(AppError::UnHandledError(format!(
                            "res_recv.try_recv error: {}",
                            err
                        ))),
                        Ok(result) => {
                            println!("Result received!");
                            match result {
                                DbResponse::Err(err) => {
                                    println!("Error result received!");
                                    Err(AppError::UnHandledError(err))
                                }
                                DbResponse::Success(result) => {
                                    println!("Success result received!");
                                    Ok(result)
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Failed to send request!");
                    Err(AppError::UnHandledError(format!(
                        "req_send.send error: {}",
                        err
                    )))
                }
            }
        }
        None => {
            println!("Unable to acquire lock!");
            Err(AppError::UnHandledError(
                "Unable to acquire Mutex lock".to_string(),
            ))
        }
    }
}
