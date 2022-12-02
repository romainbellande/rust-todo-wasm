use std::rc::Rc;
use yewdux::prelude::*;

#[derive(Store, Debug, Default, Clone, PartialEq, Eq)]
pub struct AppStore {
    pub access_token: String,
}

pub enum Action {
    SetAccessToken(String),
}

impl Reducer<AppStore> for Action {
    fn apply(self, mut store: Rc<AppStore>) -> Rc<AppStore> {
        let state = Rc::make_mut(&mut store);

        match self {
            Self::SetAccessToken(access_token) => state.access_token = access_token,
        };

        store
    }
}

// use std::rc::Rc;
// use yew::prelude::*;
//
// pub enum StoreAction {
//     SetToken(String)
// }
//
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct AppState {
//     token: Option<String>,
// }
//
// impl Reducible for AppState {
//     type Action = StoreAction;
//
//     fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
//         match action {
//             Self::Action::SetToken(token) => {
//
//                 Self {
//                     token: Some(token),
//                 }.into()
//             }
//         }
//     }
// }
//
// impl AppState {
//     pub fn new() -> Self {
//         Self { token: None }
//     }
//
//     pub fn set_token(mut self, token: Option<String>) {
//         self.token = token;
//     }
// }
//
// pub type StoreContext = UseReducerHandle<AppState>;
//
// #[derive(Properties, PartialEq)]
// pub struct Props {
//     #[prop_or_default]
//     pub children: Children,
// }
//
// #[function_component(Store)]
// pub fn store(props: &Props) -> Html {
//     let app_state = use_reducer(|| AppState::new());
//
//     html! {
//         <ContextProvider<StoreContext> context={app_state}>
//             {for props.children.iter()}
//         </ContextProvider<StoreContext>>
//     }
// }
