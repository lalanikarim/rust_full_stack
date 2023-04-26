use axum::{
    extract::{Path, State},
    Json,
};
use models::{forms::EditPersonForm, Person, Thing};

use crate::api::{AppError, AppState};

pub struct Persons;

impl Persons {
    pub async fn person(
        State(AppState { db }): State<AppState>,
        Path((id,)): Path<(String,)>,
    ) -> Result<Json<Option<Person>>, AppError> {
        let thing = Thing::from(id);
        let id = thing.id;
        let person: Option<Person> = db.select(("persons", id.to_string())).await?;
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
        Json(form): Json<EditPersonForm>,
    ) -> Result<Json<Option<Person>>, AppError> {
        let AppState { db } = state;
        let EditPersonForm { name } = form;
        let person: Option<Person> = db
            .create("persons")
            .content(Person { name, id: None })
            .await?;
        Ok(Json::from(person))
    }

    pub async fn update(
        State(state): State<AppState>,
        Path((id,)): Path<(String,)>,
        Json(form): Json<EditPersonForm>,
    ) -> Result<Json<Option<Person>>, AppError> {
        let AppState { db } = state;
        let EditPersonForm { name } = form;
        let mut response = db
            .query(format!("UPDATE persons set name = $name where id = '{id}'"))
            .bind(("name", name))
            .await?;
        let persons = response.take::<Vec<Person>>(0)?;
        let person = persons.first().map(|p| p.to_owned());
        Ok(Json::from(person))
    }

    pub async fn delete(
        State(state): State<AppState>,
        Path((id,)): Path<(String,)>,
    ) -> Result<(), AppError> {
        let AppState { db } = state;
        let id = Thing::from(id).id.to_string();
        db.delete(("persons", id)).await?;
        Ok(())
    }
}
