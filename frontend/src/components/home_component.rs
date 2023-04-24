use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(HomeComponent)]
pub fn home_component() -> Html {
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
