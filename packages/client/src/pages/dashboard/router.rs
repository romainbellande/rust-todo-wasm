use super::todos;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    TodoList,

    #[at("/create")]
    TodoCreate,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::TodoList => html! { <todos::List /> },
        Route::TodoCreate => html! { <todos::Create /> },
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
            <Switch<Route> render={Switch::render(switch)} />
    }
}
