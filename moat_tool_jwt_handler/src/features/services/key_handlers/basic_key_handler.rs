use std::fs;

use crate::models::key::Keys;

use super::key_handler::{KeyHandler, PublicKeyHandler};

#[derive(Clone)]
pub struct BasicKeyHandler {
    keys: Keys,
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
    fn get_keys(&self) -> &Keys {
        &self.keys
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
