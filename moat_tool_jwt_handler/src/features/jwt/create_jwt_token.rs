use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::models::{claims::Claims, error::Error, permission::Permission, role::Role};

pub fn create_jwt_token(
    uid: &str,
    role: Role,
    permissions: Vec<Permission>,
    private_key: &str,
    kid: &str
) -> Result<String, Error> {
    let expiration: i64 = Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("valid timestamp")
        .timestamp();

    let claims: Claims = Claims {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
        permissions: permissions
    };

    //Keys should be reused in every call
    let mut token_header = Header::new(Algorithm::RS512);
    token_header.kid = Some(kid.into());
    
    encode(
        &token_header,
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap(),
    )
    .map_err(|_| Error::JWTTokenCreationError)
}
