const COULD_NOT_SAVE_USER: &str = "COULD_NOT_SAVE_USER";
const NOT_FOUND: &str = "NOT_FOUND";

pub enum Error {
    CouldNotSaveUser,
    NotFound,
}

impl Error {
    pub fn to_vec() -> Vec<Error> {
        vec![Self::CouldNotSaveUser, Self::NotFound]
    }

    fn from_string(value: String) -> Option<Self> {
        Self::to_vec().into_iter().find_map(|item| {
            if item.to_string() == value {
                Some(item)
            } else {
                None
            }
        })
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        let str = match self {
            Self::CouldNotSaveUser => COULD_NOT_SAVE_USER,
            Self::NotFound => NOT_FOUND,
        };

        str.to_string()
    }
}
