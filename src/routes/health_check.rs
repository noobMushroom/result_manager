use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/health")]
pub async fn health(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}