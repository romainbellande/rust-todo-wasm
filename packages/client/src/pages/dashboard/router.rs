
use yew::prelude::*;
use yew_router::prelude::*;
use super::dashboard::Dashboard;
use super::todos::CreateTodo;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    CreateTodo
}

fn switch(route: &Route) -> Html {
    match route {
        Route::CreateTodo => html! { <CreateTodo /> }
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

