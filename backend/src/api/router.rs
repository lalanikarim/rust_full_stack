use axum::{routing::get, Router};

use super::route_handlers::Persons;
use super::AppState;

pub fn get_routes() -> Router<AppState> {
    let router = Router::new().nest(
        "/persons",
        Router::new()
            .route("/", get(Persons::persons).post(Persons::create))
            .route("/:id", get(Persons::person).patch(Persons::update)),
    );
    router
}
