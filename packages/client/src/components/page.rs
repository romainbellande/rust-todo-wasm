use super::{Breadcrumb, PageError, Spinner};
use shared::errors::AppError;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageProps {
    #[prop_or_default]
    pub children: Children,
    pub breadcrumb: Vec<&'static str>,

    #[prop_or(false)]
    pub loading: bool,

    #[prop_or(None)]
    pub error: Option<AppError>,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    html! {
        <div class="py-4 px-10 min-h-screen flex flex-col">
            <Breadcrumb items={props.breadcrumb.clone()} />
            <div class="px-10 py-4 grow relative flex">
                if !props.loading && props.error.is_none() {
                    {for props.children.iter()}
                } else if let Some(error) = props.error.clone() {
                    <PageError {error} />
                } else {
                    <Spinner class="absolute top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2" />
                }
            </div>
        </div>
    }
}
