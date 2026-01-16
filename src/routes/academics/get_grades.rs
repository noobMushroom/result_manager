use crate::errors::AppError;
use actix_web::{HttpResponse, get, web};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GradeBodyResponse {
    pub id: Uuid,
    pub name: String,
}

#[tracing::instrument(name = "getting the grades from db", skip(pool))]
#[get("/grades")]
pub async fn get_grades(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let grades = sqlx::query_as!(
        GradeBodyResponse,
        r#"
        SELECT id, name
        FROM grades
        ORDER BY sort_order
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!(error=?e, "Error loading grades");
        AppError::Internal
    })?;

    Ok(HttpResponse::Ok().json(grades))
}
