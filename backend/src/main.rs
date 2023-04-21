use std::{net::SocketAddr, sync::Arc};

use api::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use db::{DbClient, DbRequest};
use models::Person;
pub use surrealdb::sql::Thing;

use std::sync::mpsc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{opt::auth::Root, Surreal};
use tokio::sync::Mutex;

use crate::api::AppError;
use crate::db::{DbAction, DbResponse};

mod api {
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
}

mod db {
    use std::sync::mpsc;

    use models::Person;
    use surrealdb::{
        engine::remote::ws::{Client, Ws},
        opt::auth::Root,
        Surreal,
    };
    use tokio::sync::Mutex;

    #[derive(Debug)]
    pub enum DbAction {
        CreatePerson(Person),
        GetPerson(String),
        GetAllPersons,
    }

    #[derive(Debug)]
    pub struct DbRequest {
        pub action: DbAction,
        pub responder: mpsc::Sender<DbResponse>,
    }

    #[derive(Debug)]
    pub enum DbResponse {
        Success(Vec<Person>),
        Err(String),
    }
    pub struct DbClient {
        pub db: Surreal<Client>,
        pub receiver: Mutex<mpsc::Receiver<DbRequest>>,
    }

    impl DbClient {
        pub fn new(db: Surreal<Client>, receiver: Mutex<mpsc::Receiver<DbRequest>>) -> Self {
            Self { db, receiver }
        }

        pub async fn create_db() -> Surreal<Client> {
            let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
            db.signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .unwrap();
            db.use_ns("test").use_db("test").await.unwrap();
            db
        }

        pub async fn listen(&self) {
            let receiver = self.receiver.lock().await;
            loop {
                let receive = receiver.recv();
                println!("Received request!");
                match receive {
                    Ok(DbRequest { action, responder }) => {
                        let query = match action {
                            DbAction::GetAllPersons => "SELECT * FROM persons",
                            _ => "SELECT * FROM persons",
                        };
                        println!("Query DB");
                        let response = self.db.query(query).await;
                        let response: Vec<Person> = match response {
                            Ok(mut response) => {
                                println!("OK response");
                                let response: Vec<Person> = match response.take(0) {
                                    Ok(result) => {
                                        println!("Ok result - vec found");
                                        result
                                    }
                                    Err(err) => {
                                        dbg!(err);
                                        vec![]
                                    }
                                };
                                response
                            }
                            Err(err) => {
                                dbg!(&err);
                                vec![]
                            }
                        };
                        let send = responder.send(DbResponse::Success(response));
                        match send {
                            Ok(()) => println!("Response Sent!"),
                            Err(err) => {
                                println!("Failed to send response!");
                                dbg!(err);
                                ()
                            }
                        }
                    }
                    Err(err) => {
                        dbg!(err);
                        ()
                    }
                }
            }
        }
    }
}
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
