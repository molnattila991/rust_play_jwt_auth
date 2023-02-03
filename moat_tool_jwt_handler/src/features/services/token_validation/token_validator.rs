use crate::models::error::Error;
use async_trait::async_trait;

pub type TokenValidatorSafe = dyn TokenValidator + Send + Sync;

#[async_trait]
pub trait TokenValidator {
    async fn validate(&self, token: &str) -> Result<Vec<String>, Error>;
}
