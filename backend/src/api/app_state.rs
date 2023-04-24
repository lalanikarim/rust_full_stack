use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Client>,
}

impl AppState {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }
}
