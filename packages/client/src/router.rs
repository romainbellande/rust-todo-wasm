use crate::pages::dashboard::Dashboard;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Dashboard,

    #[at("/:s")]
    DashboardSub,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Dashboard => html! { <Dashboard /> },
        Route::DashboardSub => html! { <Dashboard /> },
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
