macro_rules! graphql_error {
    ($e:expr) => {{
        use async_graphql::ErrorExtensions;
        use crate::utils::errors::WebError;

        let web_error: WebError = $e.into();
        let graphql_error: async_graphql::Error = web_error.extend();
        graphql_error
    }};
}

pub(crate) use graphql_error;

