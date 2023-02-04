use actix_web::{dev::ServiceRequest, web::Data, Error, error::ErrorUnauthorized};

use crate::features::services::{
    key_handlers::remote_key_handler::RemoteKeyHandler,
    token_validation::remote_url_token_validator::RemoteUrlTokenValidator,
};

use super::tools::key_based_validator::key_based_validator;

pub async fn guard_remote_url(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let headers = _req.headers();
    let token_validator = _req
        .app_data::<Data<RemoteUrlTokenValidator<RemoteKeyHandler>>>()
        .unwrap()
        .as_ref();
    
    match key_based_validator(token_validator, headers).await {
        Ok(roles) => Ok(roles),
        Err(e) => {
            println!("{}", e);
            println!("Endpoint is blocked! Use valid bearer token...");
            Err(ErrorUnauthorized("Endpoint blocked!"))
        }
    }
}
