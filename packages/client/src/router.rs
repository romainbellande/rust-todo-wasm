use crate::components::AuthGuard;
use crate::pages::{Dashboard, Login};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Dashboard,

    #[at("/:s")]
    DashboardSub,

    #[at("/login")]
    Login,
}

fn render_dashboard() -> Html {
    html! {
        <AuthGuard>
            <Dashboard />
        </AuthGuard>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => render_dashboard(),
        Route::DashboardSub => render_dashboard(),
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
