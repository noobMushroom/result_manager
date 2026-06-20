use crate::{cache::app_cache::AppCache, errors::AppError, repostiory::student_repo::StudentRepo};
use actix_web::{HttpResponse, get, web};
use chrono::NaiveDate;
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

#[derive(serde::Deserialize)]
pub struct SearchStudentsQuery {
    pub grade: Option<String>,
    pub name: Option<String>,
    pub admission_no: Option<i32>,
    pub section: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl SearchStudentsQuery {
    pub fn has_filters(&self) -> bool {
        self.name.is_some() || self.grade.is_some() || self.admission_no.is_some()
    }
}

/// Search students or get all students if no parameter is given
#[tracing::instrument(
    name = "getting the Students from db",
    skip(student_repo, app_cache, query)
)]
#[get("/get_students")]
pub async fn get_students(
    student_repo: web::Data<StudentRepo>,
    query: web::Query<SearchStudentsQuery>,
    app_cache: web::Data<AppCache>,
) -> Result<HttpResponse, AppError> {
    let students = student_repo.get_students(&query, &app_cache).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "students": students
    })))
}
