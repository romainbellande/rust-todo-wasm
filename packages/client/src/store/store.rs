use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AuthState {
    token: Option<String>,
}

impl AuthState {
    pub fn new() -> Self {
        Self { token: None }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Store)]
pub fn store(props: &Props) -> Html {
    let auth = use_memo(|_| AuthState::new(), ());

    html! {
        <ContextProvider<Rc<AuthState>> context={auth}>
            {for props.children.iter()}
        </ContextProvider<Rc<AuthState>>>
    }
}
