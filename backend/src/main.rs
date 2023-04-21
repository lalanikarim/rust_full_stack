use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use models::Person;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Surreal<Client>>>,
}

impl AppState {
    fn new(db: Arc<Mutex<Surreal<Client>>>) -> Self {
        Self { db }
    }
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("test").use_db("test").await?;
    let app_state = AppState::new(Arc::new(Mutex::new(db)));
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
    Ok(())
}

async fn root() -> &'static str {
    "Hello World"
}

async fn ping() -> (StatusCode, Json<String>) {
    (StatusCode::OK, Json::from("pong".to_string()))
}

#[axum_macros::debug_handler]
async fn persons(State(_state): State<AppState>) -> Result<Json<Vec<Person>>, AppError> {
    //let db = state.db;
    //let db = db.lock().unwrap();
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("test").use_db("test").await?;
    let mut response = db.query("SELECT * FROM persons").await?;
    let response: Vec<Person> = response.take(0)?;
    //match response {
    //    Ok(resp) => (),
    //    Err(_) => (),
    //}
    //(
    //    StatusCode::OK,
    Ok(Json::from(response)) //,
                             //)
}

enum AppError {
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
