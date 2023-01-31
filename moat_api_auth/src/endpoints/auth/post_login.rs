use actix_web::{get, web, HttpResponse};
use moat_tool_jwt_handler::{
    features::{
        jwt::create_jwt_token::create_jwt_token, services::key_handlers::key_handler::KeyHandler,
    },
    models::{permission::Permission, role::Role},
};
use serde::{Deserialize, Serialize};

#[get("/login/{name}")]
pub async fn login(config: web::Data<dyn KeyHandler>, path: web::Path<String>) -> HttpResponse {
    let hash_key = config.get_keys().get_latest_key();

    let path = path.into_inner();
    let token = create_jwt_token(
        &path,
        Role::User,
        vec![Permission::Create, Permission::Read],
        &hash_key.private_key,
        &hash_key.kid,
    );

    HttpResponse::Ok().json(LoginResponse {
        access_token: token.unwrap(),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}
