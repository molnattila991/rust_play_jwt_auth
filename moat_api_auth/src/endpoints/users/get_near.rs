use actix_web::{get, HttpResponse};
use actix_web_grants::proc_macro::has_any_permission;

#[get("/near")]
#[has_any_permission["User"]]
pub async fn get_user_protected() -> HttpResponse {
    HttpResponse::Ok().body("success")
}
