
use crate::router::Router;


use yew::prelude::*;


#[function_component(App)]
pub fn app() -> Html {
    // use_effect(move || {
    //     spawn_local(async {
    //         let _result = TodosQuery::send(TodosQueryPayload { limit: Some(20) }).await.map_err(|err| {
    //             log::error!("error {:?}", err);
    //         });
    //
    //         log::debug!("test");
    //     });
    //
    //     || ()
    // });

    html! {
        <Router />
    }
}
