use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    features::{
        jwt::{
            decode_jwt_token::decode_jwt_token, decode_jwt_token_header::decode_jwt_token_header,
        },
        services::key_handlers::key_handler::{KeyHandler, PublicKeyHandler},
    },
    models::error::Error,
};

use super::token_validator::TokenValidator;

#[derive(Clone)]
pub struct BasicTokenValidator {
    key_handler: Arc<dyn PublicKeyHandler + Send + Sync>,
}

impl BasicTokenValidator {
    pub fn init(
        key_handler: Arc<dyn PublicKeyHandler + Send + Sync>,
    ) -> BasicTokenValidator {
        BasicTokenValidator {
            key_handler: key_handler,
        }
    }
}

#[async_trait]
impl TokenValidator for BasicTokenValidator {
    async fn validate(&self, token: &str) -> Result<Vec<String>, Error> {
        let kid = decode_jwt_token_header(token)?;

        let decoded_token = decode_jwt_token(token, &self.key_handler.get_public_key_by_id(&kid)?)?;
        let role_list = vec![decoded_token.role.to_string()];

        Ok(role_list)
    }
}
