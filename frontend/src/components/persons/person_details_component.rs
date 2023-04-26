use std::rc::Rc;

use gloo_net::http::Request;
use models::{forms::EditPersonForm, Person};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{components::persons::PersonComponent, routes::Route, BASE_URL};
#[derive(Properties, PartialEq)]
pub struct PersonDetailsProps {
    pub id: String,
}

#[function_component(PersonDetailsComponent)]
pub fn person_details_component(PersonDetailsProps { id }: &PersonDetailsProps) -> Html {
    let id = id.clone();
    let person = use_state(|| None);
    let person_name = Rc::new(use_state(|| None));
    let navigator = Rc::new(use_navigator().unwrap());
    let delete = {
        let id = id.clone();
        let navigator = Rc::clone(&navigator);
        move |_| {
            let navigator = navigator.clone();
            let id = id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("{BASE_URL}/api/persons/{id}");
                match Request::delete(&url).send().await {
                    Ok(_) => navigator.push(&Route::Home),
                    Err(err) => log::error!("Error received: {}", err.to_string()),
                }
            });
        }
    };
    let change = {
        let person_name = Rc::clone(&person_name);
        move |e: Event| {
            let person_name = person_name.clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_name = input.value();
            if new_name.len() > 0 {
                person_name.set(Some(new_name));
            } else {
                person_name.set(None);
            }
        }
    };
    let save = {
        let person_name = Rc::clone(&person_name);
        let navigator = Rc::clone(&navigator);
        let id = id.clone();
        move |_| {
            let navigator = navigator.clone();
            let id = id.clone();
            let person_name = person_name.clone();
            let name = &**person_name;
            let name = name.to_owned();
            if let Some(name) = name {
                wasm_bindgen_futures::spawn_local(async move {
                    let name = name.clone();
                    let id = id.clone();
                    let url = format!("{BASE_URL}/api/persons/{id}");
                    let name = name.to_string();
                    let body = EditPersonForm { name };
                    match Request::patch(&url).json(&body).unwrap().send().await {
                        Ok(_) => navigator.push(&Route::Home),
                        Err(err) => log::error!("Error received: {}", err.to_string()),
                    }
                });
            }
        }
    };
    {
        let person = person.clone();
        use_effect_with_deps(
            move |_| {
                let person = person.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let id = id.clone();
                    let url = format!("{BASE_URL}/api/persons/{id}");

                    match Request::get(&url).send().await {
                        Ok(resp) => {
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
            <>
                <PersonComponent person={person.clone()} />
                <div>
                    <span>{"Change: "}</span>
                    <span>
                        <input onchange={change}/>
                        <button onclick={save}>{"Save"}</button>
                    </span>
                </div>
                <button onclick={delete}>{"X"}</button>
            </>
        }
    } else {
        html! {
            <span>{"Not found"}</span>
        }
    }
}
