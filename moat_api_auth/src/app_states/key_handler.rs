use std::sync::Arc;

use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web::Data,
    App, Error,
};

use moat_tool_jwt_handler::features::services::key_handlers::{
    basic_key_handler::BasicKeyHandler, key_handler::KeyHandler,
};

pub fn add_key_handler<T>(app: App<T>, key_handler: BasicKeyHandler) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    let key_handler_arc: Arc<dyn KeyHandler> = Arc::new(key_handler);
    let key_handler_data: Data<dyn KeyHandler> = Data::from(key_handler_arc);

    app.app_data(key_handler_data.clone())
}
