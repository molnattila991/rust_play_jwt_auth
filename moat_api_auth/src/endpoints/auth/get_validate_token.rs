use actix_web::{get, HttpResponse};
use actix_web_grants::proc_macro::has_any_permission;
use moat_tool_jwt_handler::features::services::token_validation::remote_url_token_validator::TokenValidationResponse;

#[get("/validation")]
#[has_any_permission["User"]]
pub async fn get_validate_token() -> HttpResponse {
    println!("DB token validation logic goes here.");
    
    HttpResponse::Ok().json(TokenValidationResponse{
        token_is_valid: true
    })
}