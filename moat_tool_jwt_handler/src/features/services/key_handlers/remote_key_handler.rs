use std::{sync::Arc, future::Future};

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::models::jwk::JWK;

use super::key_handler::PublicKeyHandler;

#[derive(Clone)]
pub struct KeyToResolve {
    pub kid: String,
    pub public_key: String,
    pub exparation: String,
}

#[derive(Clone)]
pub struct RemoteKeyHandler {
    keys: Vec<KeyToResolve>,
}

impl RemoteKeyHandler {
    pub async fn init(jwk_endpoint_url: &str, http_client: Arc<Client>) -> RemoteKeyHandler {
        let mut keys: Vec<KeyToResolve> = Vec::new();

        let response = http_client.get(jwk_endpoint_url).send().await;
        let response = serde_json::from_str::<JWKRequest>(&response.unwrap().text().await.unwrap());
        match response {
            Ok(body) => {
                for jwk in body.jwk {
                    keys.push(KeyToResolve {
                        kid: jwk.kid,
                        public_key: jwk.modulus,
                        exparation: jwk.exp,
                    })
                }
            }
            Err(_) => {
                println!("Error during serialization");
            }
        }

        RemoteKeyHandler { keys }
    }

    pub async fn init2(get_jwk_response: impl Future<Output = Result<String, String>>) -> RemoteKeyHandler {
        let mut keys: Vec<KeyToResolve> = Vec::new();

        let response = get_jwk_response.await;
        let response = serde_json::from_str::<JWKRequest>(&response.unwrap());
        match response {
            Ok(body) => {
                for jwk in body.jwk {
                    keys.push(KeyToResolve {
                        kid: jwk.kid,
                        public_key: jwk.modulus,
                        exparation: jwk.exp,
                    })
                }
            }
            Err(_) => {
                println!("Error during serialization");
            }
        }

        RemoteKeyHandler { keys }
    }
}

impl PublicKeyHandler for RemoteKeyHandler {
    fn get_public_key_by_id(&self, kid: &str) -> Result<String, crate::models::error::Error> {
        for jwk in &self.keys {
            if jwk.kid == kid {
                return Ok(jwk.public_key.clone());
            }
        }
        Err(crate::models::error::Error::KeyNotFoundError)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct JWKRequest {
    pub jwk: Vec<JWK>,
}
