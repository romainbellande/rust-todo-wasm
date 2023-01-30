use crate::router::Route;
use crate::store::{AppStore};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthGuard)]
pub fn auth_guard(props: &Props) -> Html {
    let (state, _dispatch) = use_store::<AppStore>();
    let navigator = use_navigator().unwrap();

    if state.access_token.is_none() {
        navigator.push(&Route::Login);
    }

    html! {
        if state.access_token.is_some() {
            <>
            {for props.children.iter()}
            </>
        }
    }
}
