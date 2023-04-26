use gloo_net::http::Request;
use models::Person;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::Route, BASE_URL};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersonCreatePost {
    pub name: String,
}

#[function_component(PersonCreateComponent)]
pub fn person_create_component() -> Html {
    let state = use_state(|| None);
    let navigator = use_navigator().unwrap();
    let onchange = {
        let state = state.clone();
        move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let name = input.value();
                state.set(Some(name));
            } else {
                log::error!("Not able to get element");
            }
        }
    };
    let submit = {
        let state = state.clone();
        move |_| {
            let navigator = navigator.clone();
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("{BASE_URL}/api/persons");
                let state_value = &*state;
                if let Some(name) = state_value {
                    let name = name.to_string();
                    let body = PersonCreatePost { name };
                    let navigator = navigator.clone();
                    match Request::post(&url).json(&body).unwrap().send().await {
                        Ok(result) => {
                            match result.json().await {
                                Ok(Person { id, .. }) => {
                                    //let id = id_from_thing(id).unwrap();
                                    let id = id.expect("Id expected").to_string();
                                    navigator.push(&Route::Person { id });
                                }
                                Err(err) => log::error!("{:?}", err),
                            }
                        }
                        Err(err) => {
                            log::error!("{:?}", err);
                        }
                    };
                }
            });
        }
    };
    html! {
        <>
            <span>{"Name: "}</span>
                <input type="text" onchange={onchange} />
            <button onclick={submit}>{"Save"}</button>
        </>
    }
}
