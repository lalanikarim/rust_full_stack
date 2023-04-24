pub mod api;
pub mod db;

use api::{AppState, AxumApp};
use axum::extract::Path;
use axum::{extract::State, routing::get, Json, Router};
use db::DbClient;
use models::Person;

use crate::api::AppError;
use crate::db::DbConfig;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    let db_config = DbConfig {
        db_url: dotenv!("SURREALDB_URL"),
        db_username: dotenv!("SURREALDB_USERNAME"),
        db_password: dotenv!("SURREALDB_PASSWORD"),
        db_ns: dotenv!("SURREALDB_NS"),
        db_name: dotenv!("SURREALDB_DATABASE"),
    };
    let api_addr = dotenv!("API_LISTEN_ON");
    let db = DbClient::create(db_config).await;
    let state = AppState::new(db);
    let routes: Router<AppState> = Router::new()
        .route("/persons", get(persons))
        .route("/persons/:id", get(person));
    let server = AxumApp::create(routes, state, api_addr);
    server.await.unwrap();
}

async fn person(
    State(AppState { db }): State<AppState>,
    Path((id,)): Path<(String,)>,
) -> Result<Json<Option<Person>>, AppError> {
    let person: Option<Person> = db.select(("persons", id)).await?;
    Ok(Json::from(person))
}

async fn persons(State(AppState { db }): State<AppState>) -> Result<Json<Vec<Person>>, AppError> {
    let mut response = db.query("SELECT * from persons").await?;
    let persons: Vec<Person> = response.take(0)?;
    Ok(Json::from(persons))
}
