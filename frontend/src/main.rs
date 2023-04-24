use gloo_net::http::Request;
use models::Person;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;

#[macro_use]
extern crate dotenv_codegen;

static BASE_URL: &str = dotenv!("API_BASE_URL");

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/person/:id")]
    Person { id: String },
}

fn switch(routes: Route) -> Html {
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
            <Home/>
            </>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(PersonsComponent)]
fn persons_component() -> Html {
    let persons = use_state(|| None);
    {
        let persons = persons.clone();
        use_effect_with_deps(
            move |_| {
                let persons = persons.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("{BASE_URL}/api/persons");

                    log::info!("Making Request");
                    match Request::get(&url).send().await {
                        Ok(resp) => {
                            log::info!("Response Received");
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

#[derive(Properties, PartialEq)]
struct PersonDetailsProps {
    id: String,
}

#[function_component(PersonDetailsComponent)]
fn person_details_component(PersonDetailsProps { id }: &PersonDetailsProps) -> Html {
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

#[derive(Properties, PartialEq)]
struct PersonProps {
    person: Person,
}

#[function_component(PersonComponent)]
fn person_component(PersonProps { person }: &PersonProps) -> Html {
    let id = &person.id;
    let id = id_from_thing(id.clone()).unwrap();
    html! {
        <>
        <div>
        <span>{"Name: "}</span>
        <span>{ person.name.clone() }</span>
        </div>
        <div>
        <span>{"Id: "}</span>
        <Link<Route> to={Route::Person { id: id.clone() }}>{ &id }</Link<Route>>
        </div>
        </>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PersonCreatePost {
    name: String,
}

#[function_component(PersonCreateComponent)]
fn person_create_component() -> Html {
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
                    log::info!("{:?}", body);
                    match Request::post(&url).json(&body).unwrap().send().await {
                        Ok(result) => {
                            log::info!("{:?}", result);
                            match result.json().await {
                                Ok(Person { id, .. }) => {
                                    let id = id_from_thing(id).unwrap();
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

#[function_component(Home)]
fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let go_home = {
        move |_| {
            navigator.push(&Route::Home);
        }
    };
    html! {
        <div>
        <button onclick={go_home}>{"Home"}</button>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

fn id_from_thing(id: Option<Value>) -> Option<String> {
    if let Some(id) = id {
        if let Some(id) = id.get("id") {
            if let Some(id) = id.get("String") {
                id.as_str().map(String::from)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
