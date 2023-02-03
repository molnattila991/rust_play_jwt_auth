use actix_web::{web::Data, App, HttpServer};

mod endpoints;

use dotenv::dotenv;
use moat_tool_jwt_handler::features::services::{
    key_handlers::{basic_key_handler::BasicKeyHandler, key_handler::KeyHandler},
    token_validation::{
        basic_token_validator::BasicTokenValidator, token_validator::TokenValidatorTrait,
    },
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let key_handler = BasicKeyHandler::init("./keys.json");
    let key_handler_arc = Arc::new(key_handler.clone());
    let token_validator = BasicTokenValidator::init(key_handler_arc.clone());
    let token_validator_arc: Arc<TokenValidatorTrait> = Arc::new(token_validator.clone());

    HttpServer::new(move || {
        let app = App::new()
            .configure(crate::endpoints::auth::endpoints)
            .configure(crate::endpoints::users::endpoints);

        app.app_data(Data::from(token_validator_arc.clone()))
            .app_data(Data::from(key_handler_arc.clone()))
            .app_data(Data::from(
                key_handler_arc.clone() as Arc<dyn KeyHandler + Send + Sync>
            ))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
