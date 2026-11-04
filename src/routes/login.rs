use actix_web::{HttpResponse, http::header::ContentType};
pub mod post;

pub async fn login_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login/login.html"))
}
