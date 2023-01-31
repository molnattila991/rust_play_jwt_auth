use crate::{
    features::{
        jwt::decode_jwt_token::decode_jwt_token,
        services::key_handlers::key_handler::PublicKeyHandler,
    },
    models::error::Error,
};

pub fn validate_token(
    key_handler: impl PublicKeyHandler,
    kid: String,
    token: &str,
) -> Result<Vec<String>, Error> {
    let public_key = key_handler.get_public_key_by_id(&kid)?;
    let decoded_token = decode_jwt_token(token, &public_key)?;
    Ok(vec![decoded_token.role.to_string()])
}
