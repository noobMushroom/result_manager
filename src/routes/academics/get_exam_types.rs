use actix_web::{HttpResponse, get, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, routes::academics::ExamTypes};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExamTypeResponse {
    id: Uuid,
    name: String,
    code: ExamTypes,
}

#[tracing::instrument(name = "getting the Exam types from db", skip(pool))]
#[get("/exam_types")]
pub async fn get_exam_types(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    // let grades = sqlx::query_as!(
    //     ExamTypeResponse,
    //     r#"
    //     SELECT id, name, code
    //     FROM exam_types
    //     "#
    // )
    // .fetch_all(pool.get_ref())
    // .await
    // .map_err(|e| {
    //     tracing::error!(error=?e, "Error loading grades");
    //     AppError::Internal
    // })?;

    Ok(HttpResponse::Ok().finish())
}
