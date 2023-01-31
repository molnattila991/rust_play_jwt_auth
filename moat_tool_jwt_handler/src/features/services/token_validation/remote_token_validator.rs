use std::sync::Arc;

use crate::{
    features::{
        jwt::decode_jwt_token_header::decode_jwt_token_header,
        services::key_handlers::remote_key_handler::RemoteKeyHandler,
    },
    models::error::Error,
};

use super::{token_validator::TokenValidator, validate_token::validate_token};

#[derive(Clone)]
pub struct RemoteTokenValidator {
    key_handler: Arc<RemoteKeyHandler>,
}

impl RemoteTokenValidator {
    pub fn init(key_handler: Arc<RemoteKeyHandler>) -> RemoteTokenValidator {
        RemoteTokenValidator {
            key_handler: key_handler,
        }
    }
}

impl TokenValidator for RemoteTokenValidator {
    fn validate(&self, token: &str) -> Result<Vec<String>, Error> {
        let kid = decode_jwt_token_header(token)?;
        let role_list = validate_token(self.key_handler.as_ref().clone(), kid, token)?;

        Ok(role_list)
    }
}
