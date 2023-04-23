pub mod api;
pub mod db;

use api::{AppState, AxumApp};
use axum::{extract::State, routing::get, Json, Router};
use db::{DbClient, DbResult};
use models::Person;

use crate::api::AppError;
use crate::db::{DbAction, DbConfig};

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
    let (db_client, req_send) = DbClient::create(db_config).await;
    let app_state = AppState::new(req_send);
    let db_thread = tokio::spawn(async move {
        db_client.lock().await.listen().await;
    });
    let api_thread = tokio::spawn(async {
        let routes: Router<AppState> = Router::new().route("/persons", get(persons));
        let server = AxumApp::create(routes, app_state, api_addr);
        server.await.unwrap();
    });
    let _ = api_thread.await;
    let _ = db_thread.await;
}

async fn persons(State(state): State<AppState>) -> Result<Json<Vec<Person>>, AppError> {
    if let DbResult::Persons(persons) = api::make_db_request(state, DbAction::GetAllPersons).await?
    {
        Ok(Json::from(persons))
    } else {
        Err(AppError::UnHandledError("Bad response".to_string()))
    }
}
