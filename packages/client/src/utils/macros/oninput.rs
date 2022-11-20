macro_rules! oninput {
    ($form_state:expr, $field:tt) => {{
        let form_state = $form_state.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut state = (*form_state).clone();
            state.$field.touched = true;
            state.$field.value = input.value();
            form_state.set(state.clone());
        })
    }};
}

pub(crate) use oninput;
