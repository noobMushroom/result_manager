use crate::{domain::errors::DomainError, errors::AppError};
use actix_web::{HttpResponse, post, web};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AddMarksData {
    pub student_id: Uuid,
    pub term: Uuid,
    pub marks: Vec<MarksBody>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MarksBody {
    pub subject: Uuid,
    pub exam_type: Uuid,
    pub marks: Option<i32>,
    pub grade: Option<String>,
}

impl MarksBody {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.marks.is_some() && self.grade.is_some() {
            return Err(DomainError::BadRequest(
                "Both marks and grade present".to_string(),
            ));
        }

        Ok(())
    }
}

#[post["/add_marks"]]
pub async fn add_result(
    pool: web::Data<PgPool>,
    body: web::Json<AddMarksData>,
) -> Result<HttpResponse, AppError> {
    todo!()
}
