use std::sync::Arc;

use crate::{
    features::{
        jwt::{
            decode_jwt_token::decode_jwt_token, decode_jwt_token_header::decode_jwt_token_header,
        },
        services::key_handlers::key_handler::PublicKeyHandler,
    },
    models::error::Error,
};

use super::token_validator::TokenValidator;

#[derive(Clone)]
pub struct RemoteTokenValidator {
    key_handler: Arc<dyn PublicKeyHandler>,
}

impl RemoteTokenValidator {
    pub fn init(key_handler: Arc<dyn PublicKeyHandler>) -> RemoteTokenValidator {
        RemoteTokenValidator {
            key_handler: key_handler,
        }
    }
}

impl TokenValidator for RemoteTokenValidator {
    fn validate(&self, token: &str) -> Result<Vec<String>, Error> {
        let kid = decode_jwt_token_header(token)?;

        let decoded_token = decode_jwt_token(token, &self.key_handler.get_public_key_by_id(&kid)?)?;
        let role_list = vec![decoded_token.role.to_string()];

        Ok(role_list)
    }
}
