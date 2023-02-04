use actix_web::{web::Data, App, HttpServer};

mod endpoints;

use dotenv::dotenv;
use moat_tool_jwt_handler::features::services::{
    key_handlers::{
        basic_key_handler::BasicKeyHandler,
        key_handler::{KeyHandlerSafe, PublicKeyHandlerSafe},
    },
    token_validation::basic_token_validator::BasicTokenValidator,
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let key_handler = BasicKeyHandler::init("./keys.json");
    let key_handler = Arc::new(key_handler.clone());
    let token_validator = BasicTokenValidator {
        key_handler: key_handler.clone(),
    };
    let token_validator: Arc<BasicTokenValidator<BasicKeyHandler>> =
        Arc::new(token_validator.clone());

    HttpServer::new(move || {
        let app = App::new()
            .configure(crate::endpoints::auth::endpoints)
            .configure(crate::endpoints::users::endpoints);

        app.app_data(Data::from(token_validator.clone()))
            .app_data(Data::from(key_handler.clone() as Arc<PublicKeyHandlerSafe>))
            .app_data(Data::from(key_handler.clone() as Arc<KeyHandlerSafe>))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
