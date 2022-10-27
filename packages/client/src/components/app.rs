use super::Button;
use crate::graphql::client::{TodosQuery, TodosQueryPayload};

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{use_effect, use_state, Callback};

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter;
        Callback::from(move |_| counter.set(*counter + 1))
    };

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
        <div>
            <h1 class={"bg-red-500"}>{ "Hello World" }</h1>
            <Button {onclick}>{ "My Button" }</Button>
        </div>
    }
}
