use models::Person;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct PersonProps {
    pub person: Person,
}

#[function_component(PersonComponent)]
pub fn person_component(PersonProps { person }: &PersonProps) -> Html {
    let id = person.clone().id.expect("Id missing");
    let id = id.id;
    html! {
        <>
            <div>
                <span>{"Name: "}</span>
                <span>{ person.name.clone() }</span>
            </div>
            <div>
                <span>{"Id: "}</span>
                <Link<Route> to={Route::Person { id: id.to_string() }}>{ id.to_string() }</Link<Route>>
            </div>
        </>
    }
}
