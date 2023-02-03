use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web::Data, Error};

use crate::features::services::token_validation::token_validator::TokenValidator;

use super::tools::key_based_validator::get_bearer_token;

pub async fn guard_remote_url(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let headers = _req.headers();
    let token_validator = _req.app_data::<Data<dyn TokenValidator + Send + Sync>>().unwrap();

    let token = get_bearer_token(headers);
    if token.is_err() {
        return Err(ErrorUnauthorized("Endpoint blocked!"));
    }
    let token: &str = token.unwrap();

    match token_validator.validate(&token).await {
        Ok(roles) => Ok(roles),
        Err(e) => {
            println!("{}", e);
            println!("Endpoint is blocked! Use valid bearer token...");
            Err(ErrorUnauthorized("Endpoint blocked!"))
        }
    }
}
