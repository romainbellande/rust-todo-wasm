use crate::router::Router;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Router />
    }
}
