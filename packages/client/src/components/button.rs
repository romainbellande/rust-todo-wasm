use yew::prelude::*;
use yew::{Children, Properties, events::MouseEvent};

#[derive(PartialEq)]
pub enum ButtonType {
    Button,
    Submit,
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
        }
    }
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Button
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub ty: ButtonType,

    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            type={props.ty.as_str()}
            onclick={&props.onclick}
            class={"inline-block rounded border border-indigo-600 bg-indigo-600 px-12 py-3 text-sm font-medium text-white hover:bg-transparent hover:text-indigo-600 focus:outline-none focus:ring active:text-indigo-500"}
        >
            { for props.children.iter() }
        </button>
    }
}