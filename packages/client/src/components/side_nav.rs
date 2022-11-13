use super::SideNavItem;
use crate::pages::dashboard::router::Route;
use yew::prelude::*;


#[function_component[SideNav]]
pub fn side_nav() -> Html {
    html! {
        <div class={"w-64 max-w-xs flex h-screen flex-col justify-between border-r bg-white"}>
            <div class={"px-4 py-6"}>
                <span class={"block h-10 w-32 rounded-lg"}>
                    {"Rust Todo WASM"}
                </span>
                <nav aria-label={"main nav"} class={"mt-6 flex flex-col space-y-1"}>
                    <SideNavItem<Route> to={Route::CreateTodo}>
                        { "create todo" }
                    </SideNavItem<Route>>
                </nav>
            </div>
        </div>
    }
}
