use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JWK {
    pub alg: String,
    #[serde(rename = "mod")]
    pub modulus: String,
    pub exp: String,
    pub kid: String,
}