use yew::prelude::*;
use super::SideNav;
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
            <div>
              { for props.children.iter() }
            </div>
        </div>
    }
}
