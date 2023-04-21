use models::Person;
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
    let persons = vec![Person::new("karim".into()), Person::new("semina".into())];
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
    yew::Renderer::<App>::new().render();
}
