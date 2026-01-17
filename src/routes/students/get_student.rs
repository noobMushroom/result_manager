use crate::{
    domain::errors::DomainError, errors::AppError, routes::academics::get_grades::get_grade_info,
};
use actix_web::{HttpResponse, get, web};
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetStudentsResponse {
    id: Uuid,
    name: String,
    father_name: String,
    grade_name: String,
    admission_no: i32,
    date_of_birth: NaiveDate,
}

#[tracing::instrument(name = "getting the Students from db", skip(pool))]
#[get("/get_students/{grade}")]
pub async fn get_students(
    pool: web::Data<PgPool>,
    grade: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let grade = grade.into_inner();
    let grade_id = get_grade_info(&pool, &grade.to_uppercase())
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid grade name".to_string()))?;

    let body = sqlx::query_as!(
        GetStudentsResponse,
        r#"
        SELECT
            s.id,
            s.name,
            s.father_name,
            g.name          AS grade_name,
            s.admission_no,
            s.date_of_birth
        FROM students s
        JOIN grades g
            ON s.grade_id = g.id
        WHERE s.grade_id = $1
        ORDER BY s.admission_no
        "#,
        grade_id.id,
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!(error=?e, "Error while loading students");
        AppError::Internal
    })?;

    Ok(HttpResponse::Ok().json(body))
}

#[derive(serde::Deserialize)]
pub struct SearchStudentsQuery {
    pub grade_id: Option<Uuid>,
    pub q: Option<String>,
    pub admission_no: Option<i32>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl SearchStudentsQuery {
    fn validate(&self) -> Result<(), DomainError> {
        if self.q.is_none() && self.grade_id.is_none() && self.admission_no.is_none() {
            return Err(DomainError::BadRequest("Invalid search query".to_string()));
        }

        Ok(())
    }
}

#[tracing::instrument(name = "getting the Students from db", skip(pool, query))]
#[get("/search")]
pub async fn seach_students(
    pool: web::Data<PgPool>,
    query: web::Query<SearchStudentsQuery>,
) -> Result<HttpResponse, AppError> {
    query.validate()?;

    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let students = sqlx::query_as!(
        GetStudentsResponse,
        r#"
        SELECT
            s.id,
            s.name,
            s.father_name,
            g.name AS grade_name,
            s.admission_no,
            s.date_of_birth
        FROM students s
        JOIN grades g ON s.grade_id = g.id
        WHERE
            ($1::uuid IS NULL OR s.grade_id = $1)
        AND ($2::int IS NULL OR s.admission_no = $2)
        AND (
              $3::text IS NULL
              OR s.name ILIKE '%' || $3 || '%'
              OR s.father_name ILIKE '%' || $3 || '%'
        )
        ORDER BY s.admission_no
        LIMIT $4 OFFSET $5
        "#,
        query.grade_id,
        query.admission_no,
        query.q,
        limit,
        offset,
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!(error = ?e, "Error searching students");
        AppError::Internal
    })?;

    Ok(HttpResponse::Ok().json(students))
}
