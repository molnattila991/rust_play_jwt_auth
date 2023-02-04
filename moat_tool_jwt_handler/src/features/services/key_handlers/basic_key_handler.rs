use std::fs;

use crate::models::{key::Keys, jwk::JWK};

use super::key_handler::{KeyHandler, PublicKeyHandler};

#[derive(Clone)]
pub struct BasicKeyHandler {
    pub keys: Keys,
}

impl BasicKeyHandler {
    pub fn init(key_file_path: &str) -> BasicKeyHandler {
        let contents =
            fs::read_to_string(key_file_path).expect("Should have been able to read the file");

        let mut keys: Keys = serde_json::from_str(&contents).unwrap();
        keys.sort_keys();

        BasicKeyHandler { keys }
    }
}

impl KeyHandler for BasicKeyHandler {
    fn get_jwk_keys(&self) -> Vec<JWK> {
        self.keys.get_jwk_list()
    }

    fn get_latest_key(&self) -> crate::models::key::KeyToHash {
        self.keys.get_latest_key()
    }
}

impl PublicKeyHandler for BasicKeyHandler {
    fn get_public_key_by_id(&self, kid: &str) -> Result<String, crate::models::error::Error> {       
        if let Ok(key) = self.keys.get_public_key_by_id(kid) {
            return Ok(key);
        } else {
            Err(crate::models::error::Error::KeyNotFoundError)
        }
    }
}
