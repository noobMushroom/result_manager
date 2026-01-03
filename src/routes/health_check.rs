use actix_web::{HttpRequest, HttpResponse, Responder, get};

#[get("/health")]
pub async fn health(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
