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
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::token_validator::TokenValidator;

#[derive(Clone)]
pub struct RemoteUrlTokenValidator {
    key_handler: Arc<dyn PublicKeyHandler + Send + Sync>,
    validation_url: String,
    http_client: Arc<reqwest::Client>,
}

impl RemoteUrlTokenValidator {
    pub fn init(
        key_handler: Arc<dyn PublicKeyHandler + Send + Sync>,
        http_client: Arc<reqwest::Client>,
        validation_url: String,
    ) -> RemoteUrlTokenValidator {
        RemoteUrlTokenValidator {
            key_handler: key_handler,
            validation_url: validation_url,
            http_client,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TokenValidationResponse {
    pub token_is_valid: bool,
}

#[async_trait]
impl TokenValidator for RemoteUrlTokenValidator {
    async fn validate(&self, token: &str) -> Result<Vec<String>, Error> {
        let kid = decode_jwt_token_header(token)?;
        let decoded_token = decode_jwt_token(token, &self.key_handler.get_public_key_by_id(&kid)?)?;
        let role_list = vec![decoded_token.role.to_string()];

        let response = self
            .http_client
            .get(&self.validation_url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .send()
            .await;

        if response.is_err() {
            return Err(Error::WrongCredentialsError);
        }

        let response = serde_json::from_str::<TokenValidationResponse>(
            &response.unwrap().text().await.unwrap(),
        );
        if response.unwrap().token_is_valid == true {
            return Ok(role_list);
        } else {
            Err(Error::WrongCredentialsError)
        }
    }
}
