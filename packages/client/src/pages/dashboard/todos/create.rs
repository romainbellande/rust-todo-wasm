use crate::components::{Button, ButtonType, Field, FieldDef, Page};
use crate::graphql::client::{TodosQuery, TodosQueryPayload};
use crate::utils::macros::oninput;
use validator::{StringValidator, Validator};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::use_async;

#[derive(Clone, Debug)]
pub struct CreateTodoDto {
    title: String,

    description: String,
}

#[derive(Clone)]
struct FormState {
    title: FieldDef<StringValidator>,
    description: FieldDef<StringValidator>,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            title: FieldDef::new(|value| Validator::string(value).required("title is required")),
            description: FieldDef::new(|value| {
                Validator::string(value).required("description is required")
            }),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.title.is_valid() && self.description.is_valid()
    }
}

impl Into<CreateTodoDto> for FormState {
    fn into(self) -> CreateTodoDto {
        CreateTodoDto {
            title: self.title.value.clone(),
            description: self.description.value.clone(),
        }
    }
}

#[function_component(Create)]
pub fn create() -> Html {
    let form_state = use_state(|| FormState::new());

    let onsubmit = {
        let form_state = form_state.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let dto: CreateTodoDto = (*form_state).clone().into();
            log::debug!("dto: {:?}", dto);

            use_async(async move { TodosQuery::send(TodosQueryPayload { limit: Some(20) }).await });
        })
    };

    let oninput_title = oninput!(form_state, title);

    let oninput_description = oninput!(form_state, description);

    html! {
        <Page breadcrumb={vec!["todo", "create"]}>
            <form class="flex flex-col space-y-4 max-w-md m-auto justify-center grow" {onsubmit}>
                <div class={classes!("flex", "flex-col", "space-y-4")}>
                    <Field<StringValidator>
                        placeholder={"title"}
                        oninput={oninput_title}
                        field={form_state.title.clone()} />

                    <Field<StringValidator>
                        placeholder="description"
                        oninput={oninput_description}
                        field={form_state.description.clone()}  />
                </div>

                <Button ty={ButtonType::Submit} disabled={!form_state.is_valid()}>{"submit"}</Button>
            </form>
        </Page>
    }
}
