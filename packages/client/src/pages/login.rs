use yew_router::prelude::use_navigator;
use crate::router::Route;
use crate::components::{Button, ButtonType, Field, FieldDef};
use crate::graphql::auth::login_query::Credentials;
use crate::graphql::auth::{LoginPayload, LoginQuery};
use crate::store::{Action, AppStore};
use crate::utils::macros::oninput;
use rust_i18n::t;
use shared::errors::AppError;
use validator::{StringValidator, Validator};

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;
use yewdux::prelude::*;

#[derive(Clone)]
struct FormState {
    email: FieldDef<StringValidator>,

    password: FieldDef<StringValidator>,
}

impl From<FormState> for LoginPayload {
    fn from(val: FormState) -> Self {
        let credentials = Credentials {
            email: val.email.value.clone(),
            password: val.password.value,
        };

        LoginPayload { credentials }
    }
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
    let (_store, dispatch) = use_store::<AppStore>();
    let navigator = use_navigator().unwrap();
    let form_state = use_state(FormState::new);
    let error = use_state(|| Option::<AppError>::None);

    let login_request = {
        let form_state = form_state.clone();
        let error = error.clone();

        use_async(async move {
            let dto: LoginPayload = (*form_state).clone().into();
            let dispatch = dispatch.clone();
            let result = LoginQuery::send(dto).await;

            match result {
                Ok(token_success) => {
                    let access_token = token_success.login.access_token;
                    dispatch.apply(Action::SetAccessToken(access_token));
                    navigator.push(&Route::Dashboard);
                    Ok(())
                }
                Err(token_err) => {
                    error.set(Some(token_err));
                    Err(())
                }
            }
        })
    };

    let onsubmit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            login_request.run();
        })
    };

    let oninput_email = oninput!(form_state, email);
    let oninput_password = oninput!(form_state, password);

    html! {
        <div class="w-screen h-screen flex items-center">
            <div class="mx-auto space-y-8 p-10 shadow-md">
                <h1 class="text-2xl sm:text-3xl font-bold">{"Login"}</h1>
                <p>{ t!("hello") }</p>
                <form class="space-y-6" {onsubmit}>
                    <Field<StringValidator>
                        field={form_state.email.clone()}
                        placeholder="email"
                        oninput={oninput_email}
                        class="w-80"
                    />

                    <Field<StringValidator>
                        ty="password"
                        field={form_state.password.clone()}
                        placeholder="password"
                        oninput={oninput_password}
                        class="w-80"
                    />

                        if let Some(error) = &*error.clone() {
                            <p>{ t!(&format!("errors.{}", error.to_string())) }</p>
                        }

                    <div class="flex justify-end">
                        <Button ty={ButtonType::Submit} disabled={!form_state.is_valid()}>{"submit"}</Button>
                    </div>
                </form>
            </div>
        </div>
    }
}
