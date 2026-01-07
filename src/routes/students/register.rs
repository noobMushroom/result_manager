use actix_web::{HttpRequest, HttpResponse, Responder, post};
use chrono::NaiveDate;

use crate::domain::username::Username;

#[derive(serde::Deserialize)]
pub struct StudentData {
    pub name: Username,
    pub date_of_birth: NaiveDate,
    pub admission_no: u32,
    pub father_name: Username,
    pub grade: String,
}

#[post("/add_student")]
pub async fn register_student(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
