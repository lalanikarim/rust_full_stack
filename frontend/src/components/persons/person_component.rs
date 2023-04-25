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
    let id = id.to_string();
    let link = html! {
        <Link<Route> to={Route::Person { id: id.clone() }}>{ id }</Link<Route>>
    };
    html! {
        <>
            <div>
                <span>{"Name: "}</span>
                <span>{ person.name.clone() }</span>
            </div>
            <div>
                <span>{"Id: "}</span>
                { link }
            </div>
        </>
    }
}
