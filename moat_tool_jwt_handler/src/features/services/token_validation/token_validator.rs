use crate::models::error::Error;

pub trait TokenValidator {
    fn validate(&self, token: &str) -> Result<Vec<String>, Error>;
}
