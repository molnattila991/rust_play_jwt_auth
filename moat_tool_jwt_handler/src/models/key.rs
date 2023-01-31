use super::{error::Error, jwk::JWK};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Key {
    kid: String,
    private_key: String,
    public_key: String,
    expiration: i32,
}

pub struct KeyToHash<'a> {
    pub kid: &'a str,
    pub private_key: &'a str,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Keys {
    keys: Vec<Key>,

    #[serde(skip_deserializing)]
    latest_key: Key,
}

impl Keys {
    pub fn sort_keys(&mut self) {
        self.keys.sort_by(|a, b| b.expiration.cmp(&a.expiration));

        self.latest_key = self.keys[0].clone();
    }

    pub fn get_latest_key(&self) -> KeyToHash {
        KeyToHash {
            kid: &self.latest_key.kid,
            private_key: &self.latest_key.private_key,
        }
    }

    pub fn get_public_key_by_id(&self, kid: &str) -> Result<String, Error> {
        for key in &self.keys {
            if key.kid.eq(kid) {
                return Ok(key.public_key.clone());
            }
        }

        Err(Error::KeyNotFoundError)
    }

    pub fn get_jwk_list(&self) -> Vec<JWK> {
        let mut keys: Vec<JWK> = Vec::new();
        for key in &self.keys {
            keys.push(JWK {
                alg: String::from("RSA"),
                modulus: key.public_key.clone(),
                exp: key.expiration.to_string(),
                kid: key.kid.clone(),
            });
        }

        keys
    }
}
