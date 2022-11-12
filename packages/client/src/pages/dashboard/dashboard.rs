use yew::prelude::*;
use crate::components::Layout;
use super::router::Router;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <Layout>
            <Router />
        </Layout> 
    }
}
