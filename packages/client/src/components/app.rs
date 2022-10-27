use yew::prelude::*;
use yew::{use_state, Callback, use_effect};
use wasm_bindgen_futures::spawn_local;
use log;
use crate::graphql::client::{TodosQuery, TodosQueryPayload};
use super::Button;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    use_effect(move || {
        spawn_local(async {
            let result = TodosQuery::send(TodosQueryPayload {}).await.map_err(|err| {
                log::error!("error {:?}", err);
            });
    
            log::debug!("test");
        });

        || ()
    });

    
    html! {
        <div>
            <h1 class={"bg-red-500"}>{ "Hello World" }</h1>
            <Button {onclick}>{ "My Button" }</Button>
        </div>
    }
}