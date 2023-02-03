use actix_web::{
    error::ErrorUnauthorized,
    http::header::{self, HeaderMap},
    Error,
};

use crate::features::services::token_validation::token_validator::TokenValidatorSafe;


pub async fn key_based_validator(
    token_validator: &TokenValidatorSafe,
    headers: &HeaderMap,
) -> Result<Vec<String>, Error> {
    let token = get_bearer_token(headers);
    if token.is_err() {
        return Err(ErrorUnauthorized("Endpoint blocked!"));
    }
    let token: &str = token.unwrap();

    match token_validator.validate(&token).await {
        Ok(roles) => Ok(roles),
        Err(e) => {
            println!("{}", e);
            println!("Endpoint is blocked! Use valid bearer token...");
            Err(ErrorUnauthorized("Endpoint blocked!"))
        }
    }
}

pub fn get_bearer_token(headers: &HeaderMap) -> Result<&str, Error> {
    let _auth = headers.get(header::AUTHORIZATION);
    if _auth.is_none() {
        return Err(ErrorUnauthorized("Endpoint is blocked!"));
    }

    let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
    if _split.len() != 2 {
        return Err(ErrorUnauthorized("Endpoint is blocked!"));
    }

    Ok(_split[1].trim())
}
