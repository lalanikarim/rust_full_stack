pub mod db_client;
pub mod db_config;

use std::sync::mpsc;

use models::Person;

#[derive(Debug)]
pub enum DbAction {
    CreatePerson(Person),
    GetPerson(String),
    GetAllPersons,
}

#[derive(Debug)]
pub enum DbResult {
    Person(Person),
    Persons(Vec<Person>),
}

#[derive(Debug)]
pub struct DbRequest {
    pub action: DbAction,
    pub responder: mpsc::Sender<DbResponse>,
}

#[derive(Debug)]
pub enum DbResponse {
    Success(DbResult),
    Err(String),
}

pub use db_client::DbClient;
pub use db_config::DbConfig;
