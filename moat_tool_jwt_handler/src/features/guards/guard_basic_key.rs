use actix_web::{dev::ServiceRequest, web::Data, Error};

use crate::features::services::{
    key_handlers::basic_key_handler::BasicKeyHandler,
    token_validation::basic_token_validator::BasicTokenValidator,
};

use super::tools::key_based_validator::key_based_validator;

pub async fn guard_basic_key(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let headers = _req.headers();
    let token_validator = _req
        .app_data::<Data<BasicTokenValidator<BasicKeyHandler>>>()
        .unwrap()
        .as_ref();
    key_based_validator(token_validator, headers).await
}
