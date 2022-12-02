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

