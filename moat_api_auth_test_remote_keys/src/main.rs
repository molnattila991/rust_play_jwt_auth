use std::sync::Arc;

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
        key_handlers::{key_handler::PublicKeyHandlerSafe, remote_key_handler::RemoteKeyHandler},
        token_validation::{
            basic_token_validator::BasicTokenValidator, token_validator::TokenValidatorSafe,
        },
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let http_client = Arc::new(reqwest::Client::builder().build().unwrap());
    let key_handler: Arc<PublicKeyHandlerSafe> = Arc::new(
        RemoteKeyHandler::init2(get_jwk_response(
            "http://localhost:8080/auth/jwk",
            http_client.clone(),
        ))
        .await,
    );

    let token_validator: Arc<TokenValidatorSafe> =
        Arc::new(BasicTokenValidator::init(key_handler.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(token_validator.clone()))
            .app_data(Data::from(key_handler.clone()))
            .service(
                web::scope("/users")
                    .wrap(GrantsMiddleware::with_extractor(guard_remote_url))
                    .service(get_protected),
            )
    })
    .bind(("127.0.0.1", 8091))?
    .run()
    .await
}

#[get("/protected")]
#[has_any_permission["User"]]
async fn get_protected() -> HttpResponse {
    HttpResponse::Ok().body("You can see this protected content that was validated by the centralized auth service. Try again with invalid token.")
}

async fn get_jwk_response(url: &str, http_client: Arc<reqwest::Client>) -> Result<String, String> {
    let result = http_client.get(url).send().await;
    match result {
        Ok(r) => Ok(r.text().await.unwrap()),
        Err(err) => Err(format!("{}", err)),
    }
}
