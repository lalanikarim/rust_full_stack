pub mod db_client;

use std::sync::mpsc;

use models::Person;

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

pub use db_client::DbClient;
