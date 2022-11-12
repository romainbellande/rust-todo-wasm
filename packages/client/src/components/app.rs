use crate::graphql::client::{TodosQuery, TodosQueryPayload};
use crate::router::Router;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::use_effect;

#[function_component(App)]
pub fn app() -> Html {
    use_effect(move || {
        spawn_local(async {
            let _result = TodosQuery::send(TodosQueryPayload {}).await.map_err(|err| {
                log::error!("error {:?}", err);
            });

            log::debug!("test");
        });

        || ()
    });

    html! {
        <Router />
    }
}
