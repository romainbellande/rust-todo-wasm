use super::SideNav;
use yew::prelude::*;
use yew::Children;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class={"flex"}>
            <SideNav />
            <div class="w-full">
              { for props.children.iter() }
            </div>
        </div>
    }
}
