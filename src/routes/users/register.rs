use crate::routes::domain::phone::Phone;
use crate::routes::domain::username::Username;
use actix_web::{web, HttpResponse};
#[derive(serde::Deserialize)]
struct UserData {
    username: Username,
    phone: Phone,
}


pub fn subscribe(json: web::Json<UserData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

