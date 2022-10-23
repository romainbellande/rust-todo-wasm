use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use log;
use crate::graphql::client::{TodosQuery, TodosQueryPayload};

#[function_component(App)]
pub fn app() -> Html {
    spawn_local(async {
        let result = TodosQuery::send(TodosQueryPayload {}).await.map_err(|err| {
            log::error!("error {:?}", err);
        });

        log::debug!("test");
    });
    
    html! {
        <h1 class={"bg-red-500"}>{ "Hello World" }</h1>
    }
}