use std::sync::Arc;

use crate::{
    features::{
        jwt::decode_jwt_token_header::decode_jwt_token_header,
        services::key_handlers::remote_key_handler::RemoteKeyHandler,
    },
    models::error::Error,
};
use serde::{Deserialize, Serialize};

use super::{token_validator::TokenValidator, validate_token::validate_token};

#[derive(Clone)]
pub struct RemoteUrlTokenValidator {
    key_handler: Arc<RemoteKeyHandler>,
    validation_url: String,
    http_client: Arc<reqwest::Client>,
}

impl RemoteUrlTokenValidator {
    pub fn init(
        key_handler: Arc<RemoteKeyHandler>,
        http_client: Arc<reqwest::Client>,
        validation_url: String,
    ) -> RemoteUrlTokenValidator {
        RemoteUrlTokenValidator {
            key_handler: key_handler,
            validation_url: validation_url,
            http_client,
        }
    }

    pub async fn validate(&self, token: &str) -> Result<Vec<String>, Error> {
        let kid = decode_jwt_token_header(token)?;
        let role_list = validate_token(self.key_handler.as_ref().clone(), kid, token)?;

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

#[derive(Deserialize, Serialize)]
pub struct TokenValidationResponse {
    pub token_is_valid: bool,
}

impl TokenValidator for RemoteUrlTokenValidator {
    fn validate(&self, token: &str) -> Result<Vec<String>, Error> {
        let kid = decode_jwt_token_header(token)?;
        let role_list = validate_token(self.key_handler.as_ref().clone(), kid, token)?;

        Ok(role_list)
    }
}
