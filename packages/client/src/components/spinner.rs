use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(Spinner)]
pub fn spinner(props: &Props) -> Html {
    let class = String::from(props.class);

    html! {
        <div class={classes!("lds-ring", class)}><div></div><div></div><div></div><div></div></div>
    }
}
