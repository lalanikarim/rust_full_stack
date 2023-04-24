pub mod api;
pub mod db;

use api::{AppState, AxumApp};
use db::DbClient;

use crate::db::DbConfig;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    let db_config = DbConfig::read_from_env();
    let api_addr = dotenv!("API_LISTEN_ON");
    let db = DbClient::create(db_config).await;
    let state = AppState::new(db);
    let routes = api::get_routes();
    let server = AxumApp::create(routes, state, api_addr);
    server.await.unwrap();
}
