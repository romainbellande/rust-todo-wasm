use yew::prelude::*;
use super::Breadcrumb;

#[derive(Properties, PartialEq)]
pub struct PageProps {
    #[prop_or_default]
    pub children: Children,
    pub breadcrumb: Vec<&'static str>
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    html! {
        <div class="py-4 px-10">
            <Breadcrumb items={props.breadcrumb.clone()} />
            <div class="px-10 py-4">
                { for props.children.iter() }
            </div>
        </div>
    }
}
