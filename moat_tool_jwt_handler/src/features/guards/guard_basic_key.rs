use actix_web::{
    dev::ServiceRequest,
    error::ErrorUnauthorized,
    http::header::{self, HeaderMap},
    web::Data,
};

use crate::{
    features::{
        jwt::{
            decode_jwt_token::decode_jwt_token, decode_jwt_token_header::decode_jwt_token_header,
        },
        services::key_handlers::basic_key_handler::BasicKeyHandler,
    },
    models::{claims::Claims, error::Error, key::Key},
};

pub async fn guard_basic_key(_req: &ServiceRequest) -> Result<Vec<String>, actix_web::Error> {
    let headers = _req.headers();
    let key_handler = _req.app_data::<Data<BasicKeyHandler>>().unwrap().as_ref();

    let token = get_bearer_token(headers);
    let token: &str = token.unwrap();

    let result = validate(
        decode_jwt_token_header,
        decode_jwt_token,
        get_public_key_by_id(key_handler.keys.keys.clone()),
    )(&token);

    match result {
        Ok(roles) => Ok(roles),
        Err(e) => {
            println!("{}", e);
            println!("Endpoint is blocked! Use valid bearer token...");
            Err(ErrorUnauthorized("Endpoint blocked!"))
        }
    }
}

fn validate(
    decode_header: impl Fn(&str) -> Result<String, Error>,
    decode_token: impl Fn(&str, &str) -> Result<Claims, Error>,
    get_public_key: impl Fn(&str) -> Result<String, Error>,
) -> impl Fn(&str) -> Result<Vec<String>, Error> {
    move |token: &str| {
        Ok(vec![decode_token(
            &token,
            &get_public_key(&decode_header(token)?)?,
        )?
        .role
        .to_string()])
    }
}

pub fn get_public_key_by_id(keys: Vec<Key>) -> impl Fn(&str) -> Result<String, Error> {
    move |kid: &str| {
        for key in &keys {
            if key.kid.eq(kid) {
                return Ok(key.public_key.clone());
            }
        }

        Err(Error::KeyNotFoundError)
    }
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
