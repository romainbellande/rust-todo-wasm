use yew::{function_component, html, Children, prelude::*};
use yew_router::{components::Link, Routable};

#[derive(Properties, PartialEq)]
pub struct Props<R: Routable>{
    #[prop_or_default]
    pub children: Children,

    pub to: R,
}

#[function_component(SideNavItem)]
pub fn side_nav_item<R: Routable + 'static>(props: &Props<R>) -> Html {
    html! {
        <Link<R> to={props.to.clone()}>
            <span class={"flex items-center rounded-lg px-4 py-2 text-gray-500 hover:bg-gray-100 hover:text-gray-700"}>
            { for props.children.iter() }
            </span>
        </Link<R>>
    }
}
