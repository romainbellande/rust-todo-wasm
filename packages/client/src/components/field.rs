use validator::TypeValidator;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FieldDef<T: TypeValidator> {
    pub value: String,
    pub touched: bool,
    validate_fn: fn(String) -> T,
}

impl<T: TypeValidator> FieldDef<T> {
    pub fn new(validate_fn: fn(String) -> T) -> Self {
        Self {
            value: String::new(),
            touched: false,
            validate_fn,
        }
    }

    pub fn get_error_message(&self) -> String {
        if self.is_valid() {
            String::new()
        } else {
            self.get_validator().get_error_message()
        }
    }

    pub fn class(&self) -> String {
        let class: &str = if !self.touched {
            "border-gray-200"
        } else if self.is_valid() {
            "border-green-400"
        } else {
            "border-red-400"
        };

        class.to_string()
    }

    fn get_validator(&self) -> T {
        (self.validate_fn)(self.value.clone())
    }

    pub fn is_valid(&self) -> bool {
        self.get_validator().validate()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props<T: TypeValidator + PartialEq> {
    #[prop_or("text")]
    pub ty: &'static str,

    pub oninput: Callback<InputEvent>,

    pub field: FieldDef<T>,

    #[prop_or("")]
    pub placeholder: &'static str,

    #[prop_or("")]
    pub class: &'static str,
}

#[function_component(Field)]
pub fn field<T: TypeValidator + PartialEq>(props: &Props<T>) -> Html {
    let input_class = String::from("w-full rounded-lg border text-sm");

    html! {
        <div class="flex flex-col">
            <input
                type="text"
                class={classes!(props.field.class(), input_class.clone(), props.class)}
                placeholder={props.placeholder}
                value={props.field.value.clone()}
                oninput={props.oninput.clone()}
            />

            if !props.field.is_valid() && props.field.touched {
                <span class="text-red-500">
                    {  props.field.get_error_message() }
                </span>
            }
        </div>
    }
}
