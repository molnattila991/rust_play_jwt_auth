use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use actix_web_grants::{proc_macro::has_any_permission, GrantsMiddleware};
use dotenv::dotenv;
use moat_tool_jwt_handler::features::{
    guards::guard_remote_url::guard_remote_url,
    services::{
        key_handlers::{key_handler::PublicKeyHandler, remote_key_handler::RemoteKeyHandler},
        token_validation::{
            remote_url_token_validator::RemoteUrlTokenValidator, token_validator::TokenValidator,
        },
    },
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let key_handler = RemoteKeyHandler::init("http://localhost:8080/auth/jwk").await;
    let http_client = reqwest::Client::builder();
    let http_client = Box::new(http_client.build().unwrap());

    let key_handler_arc: Arc<dyn PublicKeyHandler + Send + Sync> = Arc::new(key_handler.clone());

    let token_validator = RemoteUrlTokenValidator::init(
        key_handler_arc.clone(),
        http_client.clone(),
        String::from("http://localhost:8080/auth/tokens/validation"),
    );

    let token_validator_arc: Arc<dyn TokenValidator + Send + Sync> =
        Arc::new(token_validator.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(token_validator_arc.clone()))
            .app_data(Data::from(key_handler_arc.clone()))
            .service(
                web::scope("/users")
                    .wrap(GrantsMiddleware::with_extractor(guard_remote_url))
                    .service(get_protected),
            )
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}

#[get("/protected")]
#[has_any_permission["User"]]
async fn get_protected() -> HttpResponse {
    HttpResponse::Ok().body("You can see this protected content that was validated by the centralized auth service. Try again with invalid token.")
}
