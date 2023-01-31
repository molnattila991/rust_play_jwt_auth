use crate::models::{key::Keys, error::Error};

pub trait KeyHandler {
    fn get_keys(&self) -> &Keys;
}

pub trait PublicKeyHandler {
    fn get_public_key_by_id(&self, kid: &str) -> Result<String, Error>;
}