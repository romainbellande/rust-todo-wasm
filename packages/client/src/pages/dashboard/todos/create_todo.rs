use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::components::Breadcrumb;
use validator::Validator;

#[derive(Clone)]
pub struct CreateTodoDto {
    title: String,

    description: String,
}

impl CreateTodoDto {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new()
        }
    }
}

#[derive(Clone)]
struct Field {
    value: String,
    touched: bool,
    validate_fn: fn(String) -> bool
}

impl Field {
    pub fn new(validate_fn: fn(String) -> bool) -> Self {
        Self {
            value: String::new(),
            touched: false,
            validate_fn,
        }
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

enum FormStateField {
    Title,
    Description,
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
}

#[function_component(CreateTodo)]
pub fn create_todo() -> Html {
    let input_class = String::from("w-full rounded-lg border border-gray-200 text-sm");
    let form_state = use_state(|| FormState::new());

    let oninput_title = {
        let form_state = form_state.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut state = (*form_state).clone();
            state.title.touched = true;
            state.title.value = input.value();
            form_state.set(state);
        })
    };

    html! {
        <div>
            <Breadcrumb items={vec!["todo", "create"]} />
            <h1>{ "create todo page" }</h1>
            <form>
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
                        class={classes!(input_class)}
                        placeholder={"description"}
                        value={form_state.description.value.clone()}
                    />
                </div>
            </form>
        </div>
    }
}

