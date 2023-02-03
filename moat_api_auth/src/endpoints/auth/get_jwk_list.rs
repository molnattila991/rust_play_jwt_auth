use actix_web::{get, web, HttpResponse};
use moat_tool_jwt_handler::{
    features::services::key_handlers::key_handler::KeyHandler, models::jwk::JWK,
};
use serde::{Deserialize, Serialize};

#[get("/jwk")]
pub async fn get_jwk_list(config: web::Data<dyn KeyHandler + Send + Sync>) -> HttpResponse {
    let keys: Vec<JWK> = config.get_jwk_keys();

    HttpResponse::Ok().json(JWKResponse { jwk: keys })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JWKResponse {
    pub jwk: Vec<JWK>,
}
