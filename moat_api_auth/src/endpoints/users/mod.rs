use actix_web::web;
use actix_web_grants::GrantsMiddleware;
use moat_tool_jwt_handler::features::{guards::guard_basic_key::guard_basic_key};

use self::get_near::get_user_protected;

mod get_near;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(GrantsMiddleware::with_extractor(
                guard_basic_key,
            ))
            .service(get_user_protected),
    );
}
