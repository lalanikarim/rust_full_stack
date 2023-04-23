pub mod api;
pub mod db;

use api::{AppState, AxumApp};
use axum::{extract::State, routing::get, Json, Router};
use db::{DbClient, DbRequest};
use models::Person;
pub use surrealdb::sql::Thing;

use std::sync::mpsc;

use crate::api::AppError;
use crate::db::{DbAction, DbResponse};

#[tokio::main]
async fn main() {
    let (db_client, req_send) = DbClient::create().await;
    let app_state = AppState::new(req_send);
    let db_thread = tokio::spawn(async move {
        db_client.lock().await.listen().await;
    });
    let api_thread = tokio::spawn(async {
        let routes: Router<AppState> = Router::new().route("/persons", get(persons));
        let server = AxumApp::create(routes, app_state);
        server.await.unwrap();
    });
    let _ = api_thread.await;
    let _ = db_thread.await;
}

#[axum_macros::debug_handler]
async fn persons(State(state): State<AppState>) -> Result<Json<Vec<Person>>, AppError> {
    match state.req_send.lock().await.into() {
        Some(req_send) => {
            println!("Lock acquired");
            let (res_send, res_recv) = mpsc::channel::<DbResponse>();
            let send = req_send.send(DbRequest {
                action: DbAction::GetAllPersons,
                responder: res_send,
            });
            match send {
                Ok(()) => {
                    println!("Request sent!");
                    match res_recv.recv() {
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
                                DbResponse::Success(persons) => {
                                    println!("Success result received!");
                                    Ok(Json::from(persons))
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
