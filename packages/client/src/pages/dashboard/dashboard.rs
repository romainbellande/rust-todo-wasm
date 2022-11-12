use super::router::Router;
use crate::components::Layout;
use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <Layout>
            <Router />
        </Layout>
    }
}
