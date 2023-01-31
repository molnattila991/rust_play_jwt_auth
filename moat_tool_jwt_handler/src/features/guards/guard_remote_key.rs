use actix_web::{dev::ServiceRequest, web::Data, Error};

use crate::features::services::token_validation::remote_token_validator::RemoteTokenValidator;

use super::tools::key_based_validator::key_based_validator;

pub async fn guard_remote_key(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let headers = _req.headers();
    let token_validator = _req.app_data::<Data<RemoteTokenValidator>>().unwrap();

    key_based_validator::<RemoteTokenValidator>(token_validator, headers).await
}
