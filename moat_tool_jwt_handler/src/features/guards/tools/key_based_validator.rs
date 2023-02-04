use actix_web::http::header::{self, HeaderMap};

use crate::{
    features::services::token_validation::token_validator::TokenValidator, models::error::Error,
};

pub async fn key_based_validator(
    token_validator: &impl TokenValidator,
    headers: &HeaderMap,
) -> Result<Vec<String>, Error> {
    let token = get_bearer_token(headers);
    let token: &str = token.unwrap();

    Ok(token_validator.validate(&token).await?)
}

pub fn get_bearer_token(headers: &HeaderMap) -> Result<&str, Error> {
    let _auth = headers.get(header::AUTHORIZATION);
    if _auth.is_none() {
        return Err(Error::InvalidAuthHeaderError);
    }

    let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
    if _split.len() != 2 {
        return Err(Error::InvalidAuthHeaderError);
    }

    Ok(_split[1].trim())
}
