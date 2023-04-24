use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::HomeComponent;

use crate::components::persons::{PersonCreateComponent, PersonDetailsComponent, PersonsComponent};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/person/:id")]
    Person { id: String },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <PersonsComponent/>
                <PersonCreateComponent/>
            </>
        },
        Route::Person { id } => html! {
            <>
                <PersonDetailsComponent id={id}/>
                <HomeComponent/>
            </>
        },
    }
}
