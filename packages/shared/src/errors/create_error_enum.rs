macro_rules! create_error_enum {
    ($name:ident, [$($val:ident),*]) => {
        use std::str::FromStr;

        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $( $val, )*
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                let str = match self {
                    $(
                        Self::$val => stringify!($val),
                    )*
                };

                str.to_string()
            }
        }

        impl FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($val) => Ok(Self::$val),
                    )*
                    _ => Err("no match".to_string()),
                }
            }
        }
    };
}

pub(crate) use create_error_enum;
