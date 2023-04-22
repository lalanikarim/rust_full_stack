use models::Person;
use reqwest::get;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <PersonsComponent />
        </>
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
struct PersonProps {
    person: Person,
}

#[function_component(PersonComponent)]
fn person_component(PersonProps { person }: &PersonProps) -> Html {
    html! {
        <div>
        <span>{"Name: "}</span>
        <span>{ person.name.clone() }</span>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
