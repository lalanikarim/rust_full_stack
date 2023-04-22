pub mod api;
pub mod db;

use std::{net::SocketAddr, sync::Arc};

use api::AppState;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use db::{DbClient, DbRequest};
use models::Person;
pub use surrealdb::sql::Thing;

use std::sync::mpsc;

use tokio::sync::Mutex;

use crate::api::AppError;
use crate::db::{DbAction, DbResponse};

#[tokio::main]
async fn main() {
    let (req_send, req_recv) = mpsc::channel::<DbRequest>();
    let req_send = Arc::new(Mutex::new(req_send));
    let req_recv = Mutex::new(req_recv);
    let db = DbClient::create_db().await;
    let db_client = Mutex::new(DbClient::new(db, req_recv));
    let app_state = AppState::new(req_send);
    let db_thread = tokio::spawn(async move {
        db_client.lock().await.listen().await;
    });
    let api_thread = tokio::spawn(async {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let app = Router::new()
            .route("/", get(root))
            .route("/ping", get(ping))
            .route("/api/persons", get(persons))
            .with_state(app_state);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
    let _ = api_thread.await;
    let _ = db_thread.await;
}

async fn root() -> &'static str {
    "Hello World"
}

async fn ping() -> (StatusCode, Json<String>) {
    (StatusCode::OK, Json::from("pong".to_string()))
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
