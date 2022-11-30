use crate::pages::{Dashboard, Login};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Dashboard,

    #[at("/:s")]
    DashboardSub,

    #[at("/login")]
    Login,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html! { <Dashboard /> },
        Route::DashboardSub => html! { <Dashboard /> },
        Route::Login => html! { <Login /> },
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
