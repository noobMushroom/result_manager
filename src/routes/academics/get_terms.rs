use crate::errors::AppError;
use actix_web::{HttpResponse, get, web};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TermsResponse {
    pub id: Uuid,
    pub name: String,
}

#[tracing::instrument(name = "getting the terms from db", skip(pool))]
#[get("/terms")]
pub async fn get_terms(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    // let grades = sqlx::query_as!(
    //     TermsResponse,
    //     r#"
    //     SELECT id, name
    //     FROM terms 
    //     ORDER BY sort_order
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
