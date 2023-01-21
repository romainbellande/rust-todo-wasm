use shared::errors::AppError;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub error: AppError,
}

#[function_component(PageError)]
pub fn page_error(props: &Props) -> Html {
    let error_message = props.error.to_string();

    html! {
        <div class="h-full flex flex-col items-center mt-28">
            <h2 class="text-4xl">{"Uh-oh!"}</h2>
            <div class="mt-4 space-x-4">
                <span>{"The following error occured:"}</span>
                <span>{error_message}</span>
            </div>
        </div>
    }
}
