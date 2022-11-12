use crate::components::Page;
use validator::Validator;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug)]
pub struct CreateTodoDto {
    title: String,

    description: String,
}

#[derive(Clone)]
struct Field {
    value: String,
    touched: bool,
    validate_fn: fn(String) -> bool,
}

impl Field {
    pub fn new(validate_fn: fn(String) -> bool) -> Self {
        Self {
            value: String::new(),
            touched: false,
            validate_fn,
        }
    }

    pub fn set_value(mut self, value: String) {
        self.value = value;
        self.touched = true;
    }

    pub fn class(&self) -> String {
        let class: &str = if !self.touched {
            ""
        } else if self.is_valid() {
            "border border-green-400"
        } else {
            "border border-red-400"
        };

        class.to_string()
    }

    pub fn is_valid(&self) -> bool {
        (self.validate_fn)(self.value.clone())
    }
}

#[derive(Clone)]
struct FormState {
    title: Field,
    description: Field,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            title: Field::new(|value| Validator::string(value).required().valid()),
            description: Field::new(|value| Validator::string(value).required().valid()),
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

#[function_component(CreateTodo)]
pub fn create_todo() -> Html {
    let input_class = String::from("w-full rounded-lg border border-gray-200 text-sm");
    let form_state = use_state(|| FormState::new());

    let onsubmit = {
        let form_state = form_state.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let dto: CreateTodoDto = (*form_state).clone().into();
            log::debug!("dto: {:?}", dto);
        })
    };

    let oninput_title = {
        let form_state = form_state.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut state = (*form_state).clone();
            state.title.touched = true;
            state.title.value = input.value();
            form_state.set(state.clone());
        })
    };

    let oninput_description = {
        let form_state = form_state.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut state = (*form_state).clone();
            state.description.touched = true;
            state.description.value = input.value();
            form_state.set(state.clone());
        })
    };

    html! {
        <Page breadcrumb={vec!["todo", "create"]}>
            <form {onsubmit}>
                <div class={classes!("flex", "flex-col", "space-y-4")}>
                    <input
                        type={"text"}
                        class={classes!(input_class.clone(), form_state.title.class())}
                        placeholder={"title"}
                        value={form_state.title.value.clone()}
                        oninput={oninput_title}
                    />

                    <input
                        type={"text"}
                        class={classes!(input_class, form_state.description.class())}
                        placeholder={"description"}
                        value={form_state.description.value.clone()}
                        oninput={oninput_description}
                    />
                </div>

                <button type="submit" disabled={!form_state.is_valid()}>{"submit"}</button>
            </form>
        </Page>
    }
}
