use actix_web::{App, HttpServer};

mod app_states;
mod endpoints;

use app_states::{key_handler::add_key_handler, token_validator::add_token_validator};
use dotenv::dotenv;
use moat_tool_jwt_handler::features::services::{
    key_handlers::{basic_key_handler::BasicKeyHandler},
    token_validation::{basic_token_validator::BasicTokenValidator},
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
    // let key_handler_remote = RemoteKeyHandler::init("./keys.json").await;
    // let key_handler_arc = Arc::new(key_handler_remote.clone());
    // let token_validator = RemoteTokenValidator::init(key_handler_arc.clone());

    HttpServer::new(move || {
        let app = App::new()
            .configure(crate::endpoints::auth::endpoints)
            .configure(crate::endpoints::users::endpoints);

        let app = add_token_validator(app, token_validator.clone());
        let app = add_key_handler(app, key_handler.clone());
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
