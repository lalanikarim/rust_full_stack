use axum::{
    extract::{Path, State},
    Json,
};
use models::Person;
use serde::Deserialize;

use crate::api::{AppError, AppState};

pub struct Persons;

impl Persons {
    pub async fn person(
        State(AppState { db }): State<AppState>,
        Path((id,)): Path<(String,)>,
    ) -> Result<Json<Option<Person>>, AppError> {
        let person: Option<Person> = db.select(("persons", id)).await?;
        Ok(Json::from(person))
    }

    pub async fn persons(
        State(AppState { db }): State<AppState>,
    ) -> Result<Json<Vec<Person>>, AppError> {
        let mut response = db.query("SELECT * from persons").await?;
        let persons: Vec<Person> = response.take(0)?;
        Ok(Json::from(persons))
    }

    pub async fn create(
        State(state): State<AppState>,
        Json(form): Json<CreatePersonForm>,
    ) -> Result<Json<Option<Person>>, AppError> {
        let AppState { db } = state;
        let CreatePersonForm { name } = form;
        let person: Option<Person> = db
            .create("persons")
            .content(Person { name, id: None })
            .await?;
        Ok(Json::from(person))
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePersonForm {
    pub name: String,
}
