use std::sync::Arc;

use actix_web::{dev::{ServiceRequest, ServiceFactory}, web::Data, App, Error};
use moat_tool_jwt_handler::features::services::token_validation::{basic_token_validator::BasicTokenValidator};

pub fn add_token_validator<T>(app: App<T>, token_validator: BasicTokenValidator) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    let token_validator_arc: Arc<BasicTokenValidator> = Arc::new(token_validator);
    let token_validator_data: Data<BasicTokenValidator> = Data::from(token_validator_arc);

    app.app_data(token_validator_data.clone())
}