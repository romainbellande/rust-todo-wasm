use crate::components::{Button, ButtonType, Field, FieldDef};
use crate::utils::macros::oninput;
use validator::{StringValidator, Validator};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone)]
struct FormState {
    email: FieldDef<StringValidator>,

    password: FieldDef<StringValidator>,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            email: FieldDef::new(|value| Validator::string(value).required("email is required")),
            password: FieldDef::new(|value| {
                Validator::string(value).required("password is required")
            }),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.email.is_valid() && self.password.is_valid()
    }
}

#[function_component(Login)]
pub fn login() -> Html {
    let form_state = use_state(|| FormState::new());

    let onsubmit = {
        let form_state = form_state.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
        })
    };

    let oninput_email = oninput!(form_state, email);
    let oninput_password = oninput!(form_state, password);

    html! {
        <div class="w-screen h-screen flex items-center">
            <div class="mx-auto space-y-8 p-10 shadow-md">
                <h1 class="text-2xl sm:text-3xl font-bold">{"Login"}</h1>
                <form class="space-y-6" {onsubmit}>
                    <Field<StringValidator>
                        field={form_state.email.clone()}
                        placeholder="email"
                        oninput={oninput_email}
                        class="w-80"
                    />

                    <Field<StringValidator>
                        field={form_state.password.clone()}
                        placeholder="password"
                        oninput={oninput_password}
                        class="w-80"
                    />

                    <div class="flex justify-end">
                        <Button ty={ButtonType::Submit} disabled={!form_state.is_valid()}>{"submit"}</Button>
                    </div>
                </form>
            </div>
        </div>
    }
}
