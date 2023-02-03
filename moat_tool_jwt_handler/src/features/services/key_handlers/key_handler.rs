use crate::models::{error::Error, jwk::JWK, key::KeyToHash};

pub type KeyHandlerSafe = dyn KeyHandler + Send + Sync;
pub trait KeyHandler {
    fn get_jwk_keys(&self) -> Vec<JWK>;
    fn get_latest_key(&self) -> KeyToHash;
}

pub type PublicKeyHandlerSafe = dyn PublicKeyHandler + Send + Sync;
pub trait PublicKeyHandler {
    fn get_public_key_by_id(&self, kid: &str) -> Result<String, Error>;
}
