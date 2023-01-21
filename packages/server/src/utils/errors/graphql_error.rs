macro_rules! graphql_error {
    ($e:expr) => {{
        use crate::utils::errors::WebError;
        use async_graphql::ErrorExtensions;

        let web_error: WebError = $e.into();
        let graphql_error: async_graphql::Error = web_error.extend();
        graphql_error
    }};
}

pub(crate) use graphql_error;
