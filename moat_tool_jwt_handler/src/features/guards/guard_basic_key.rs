use actix_web::{dev::ServiceRequest, web::Data, Error};

use crate::features::services::token_validation::token_validator::TokenValidatorTrait;

use super::tools::key_based_validator::key_based_validator;

pub async fn guard_basic_key(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let headers = _req.headers();
    let token_validator = _req.app_data::<Data<TokenValidatorTrait>>().unwrap();
    key_based_validator(token_validator.as_ref(), headers).await
}
