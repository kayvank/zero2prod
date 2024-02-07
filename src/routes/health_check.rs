use actix_web::{HttpRequest, HttpResponse};

pub async fn health_check(_req: HttpRequest) -> HttpResponse {// impl Responder {
    HttpResponse::Ok().finish()
}
