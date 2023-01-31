use jsonwebtoken::decode_header;

use crate::models::error::Error;

pub fn decode_jwt_token_header(token_as_str: &str) -> Result<String, Error> {
    let decoded = decode_header(token_as_str);

    match decoded {
        Ok(header) => match header.kid {
            Some(kid) => Ok(kid),
            None => Err(Error::JWTTokenError),
        },
        Err(_) => Err(Error::WrongCredentialsError),
    }
}
