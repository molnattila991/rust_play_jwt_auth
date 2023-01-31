use actix_web::web;
use actix_web_grants::GrantsMiddleware;
use moat_tool_jwt_handler::features::guards::guard_basic_key::guard_basic_key;

mod get_jwk_list;
mod get_validate_token;
mod post_login;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(get_jwk_list::get_jwk_list)
            .service(
                web::scope("/tokens")
                    .wrap(GrantsMiddleware::with_extractor(guard_basic_key))
                    .service(get_validate_token::get_validate_token),
            )
            .service(web::scope("/users").service(post_login::login)),
    );
}
