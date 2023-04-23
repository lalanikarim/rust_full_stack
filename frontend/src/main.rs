use models::Person;
use reqwest::get;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/person/:id")]
    Person { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <PersonsComponent/>},
        Route::Person { id } => html! {<PersonDetailsComponent id={id}/>},
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
                    let url = "http://localhost:3000/api/persons";

                    log::info!("Making Request");
                    match get(url).await {
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
                    let url = format!("http://localhost:3000/api/persons/{id}");

                    log::info!("Making Request");
                    match get(url).await {
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
    let id = person
        .id
        .as_ref()
        .unwrap()
        .get("id")
        .unwrap()
        .get("String")
        .unwrap()
        .as_str()
        .unwrap();
    let id = String::from(id);
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

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
