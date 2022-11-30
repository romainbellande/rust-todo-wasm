use crate::components::Page;
use crate::graphql::client::{FindTodos, FindTodosPayload};
use yew::prelude::*;
use yew_hooks::prelude::{use_async_with_options, UseAsyncOptions};

#[function_component(List)]
pub fn list() -> Html {
    let todos_response = use_async_with_options(
        async move { FindTodos::send(FindTodosPayload { limit: Some(20) }).await },
        UseAsyncOptions::enable_auto(),
    );

    html! {
        <Page
            breadcrumb={vec!["todo", "list"]}
            loading={todos_response.loading}
            error={todos_response.error.clone()}
        >
            <ol class="divide-y grow">
                if let Some(data) = todos_response.data.clone() {
                    {
                        for data.todos.data.iter().map(|todo| {
                            html! {
                                <li class="px-2 py-4">{todo.title.clone()}</li>
                            }
                        })
                    }
                }
            </ol>
        </Page>
    }
}
