use serde::{Deserialize, Serialize};

use super::permission::Permission;


#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
    pub permissions: Vec<Permission>
}