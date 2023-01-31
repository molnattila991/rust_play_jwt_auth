use std::sync::Arc;

use crate::{
    features::{
        jwt::decode_jwt_token_header::decode_jwt_token_header,
        services::key_handlers::basic_key_handler::BasicKeyHandler,
    },
    models::error::Error,
};

use super::{token_validator::TokenValidator, validate_token::validate_token};

#[derive(Clone)]
pub struct BasicTokenValidator {
    key_handler: Arc<BasicKeyHandler>,
}

impl BasicTokenValidator {
    pub fn init(key_handler: Arc<BasicKeyHandler>) -> BasicTokenValidator {
        BasicTokenValidator {
            key_handler: key_handler,
        }
    }
}

impl TokenValidator for BasicTokenValidator {
    fn validate(&self, token: &str) -> Result<Vec<String>, Error> {
        let kid = decode_jwt_token_header(token)?;
        let role_list = validate_token(self.key_handler.as_ref().clone(), kid, token)?;

        Ok(role_list)
    }
}
