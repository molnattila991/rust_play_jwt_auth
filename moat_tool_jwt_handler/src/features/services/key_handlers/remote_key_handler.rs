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
    pub async fn init(jwk_endpoint_url: &str) -> RemoteKeyHandler {
        let client = awc::Client::default();
        let mut keys: Vec<KeyToResolve> = Vec::new();

        let response = client.get(jwk_endpoint_url).send().await;
        if let Ok(mut response) = response {
            if let Ok(body) = response.json::<JWKRequest>().await {
                for jwk in body.jwk {
                    keys.push(KeyToResolve {
                        kid: jwk.kid,
                        public_key: jwk.modulus,
                        exparation: jwk.exp,
                    })
                }
            } else {
                println!("almaaa");
            }
        } else {
            println!("almaaa");
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
