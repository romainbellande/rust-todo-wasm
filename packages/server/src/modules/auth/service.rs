use super::{
    body::AuthBody, claims::Claims, credentials::Credentials, errors::AuthError, keys::KEYS,
};
use crate::utils::errors::{graphql_error, CommonError};
use async_graphql::{Error, Result};
use entity::user;
use jsonwebtoken::{encode, Header};
use sea_orm::DatabaseConnection;
use crate::CONFIG;

pub async fn authorize(
    conn: &DatabaseConnection,
    credentials: Credentials,
) -> Result<AuthBody, Error> {
    // Check if the user sent the credentials
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredentials.into());
    }

    let my_user = user::Entity::find_by_email(&credentials.email)
        .one(conn)
        .await;

    let my_user =
        my_user.map_err(|err| graphql_error!(CommonError::InternalServerError(err.to_string())))?;

    let my_user = my_user.ok_or_else(|| graphql_error!(AuthError::WrongCredentials))?;

    // Here you can check the user credentials from a database
    if !my_user.verify_password(credentials.clone().password) {
        return Err(AuthError::WrongCredentials.into());
    }

    let token = create_access_token(my_user)?;

    Ok(AuthBody::new(token))
}

pub fn create_access_token(user: user::Model) -> Result<String, Error> {
    let claims = Claims {
        sub: user.id.to_string(),
        company: "ACME".to_owned(),
        // TODO: add roles here
        // Mandatory expiry time as UTC timestamp
        exp: CONFIG.access_token_duration.into(), // May 2033
    };

    // Create the authorization token
     encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| graphql_error!(AuthError::TokenCreation))

}
