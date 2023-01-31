use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::models::{claims::Claims, error::Error};

pub fn decode_jwt_token(token_as_string: &str, public_key: &str) -> Result<Claims, Error> {
    let decoded = decode::<Claims>(
        token_as_string,
        &DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
        &Validation::new(Algorithm::RS512),
    );

    match decoded {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err(Error::WrongCredentialsError),
    }
}
