use gloo_net::http::Request;
use models::Person;
use yew::prelude::*;

use crate::{components::persons::PersonComponent, BASE_URL};

#[derive(Properties, PartialEq)]
pub struct PersonDetailsProps {
    pub id: String,
}

#[function_component(PersonDetailsComponent)]
pub fn person_details_component(PersonDetailsProps { id }: &PersonDetailsProps) -> Html {
    let id = id.clone();
    let person = use_state(|| None);
    {
        let person = person.clone();
        use_effect_with_deps(
            move |_| {
                let person = person.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let id = id.clone();
                    let url = format!("{BASE_URL}/api/persons/{id}");

                    log::info!("Making Request");
                    match Request::get(&url).send().await {
                        Ok(resp) => {
                            log::info!("Response Received");
                            match resp.json().await {
                                Ok(json) => {
                                    let fetched_person: Person = json;
                                    person.set(Some(fetched_person));
                                }
                                Err(err) => log::error!("Error received: {}", err.to_string()),
                            };
                        }
                        Err(err) => log::error!("Error received: {}", err.to_string()),
                    }
                });
            },
            (),
        );
    }
    if let Some(person) = &*person {
        html! {
            <PersonComponent person={person.clone()} />
        }
    } else {
        html! {
            <span>{"Not found"}</span>
        }
    }
}
