use crate::{components::persons::PersonComponent, BASE_URL};
use gloo_net::http::Request;
use models::Person;
use yew::prelude::*;
#[function_component(PersonsComponent)]
pub fn persons_component() -> Html {
    let persons = use_state(|| None);
    {
        let persons = persons.clone();
        use_effect_with_deps(
            move |_| {
                let persons = persons.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("{BASE_URL}/api/persons");

                    match Request::get(&url).send().await {
                        Ok(resp) => {
                            match resp.json().await {
                                Ok(json) => {
                                    let fetched_persons: Vec<Person> = json;
                                    persons.set(Some(fetched_persons));
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
    if let Some(persons) = &*persons {
        html! {
            <>
            {
             persons
            .iter()
            .map(|person| html! {<PersonComponent person={person.clone()}/>})
            .collect::<Html>()
            }
            </>
        }
    } else {
        html! {
            <span>{"No items found"}</span>
        }
    }
}
