use std::fmt;

use crate::{domain::errors::DomainError, errors::AppError};
use actix_web::{HttpResponse, get, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum EvaluationType {
    Marks,
    Grade,
}

impl TryFrom<&str> for EvaluationType {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "MARKS" => Ok(Self::Marks),
            "GRADE" => Ok(Self::Grade),
            _ => Err(DomainError::BadRequest(
                "Invalid Evalution Type".to_string(),
            )),
        }
    }
}

impl From<String> for EvaluationType {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "MARKS" => Self::Marks,
            "GRADE" => Self::Grade,
            _ => Self::Marks,
        }
    }
}

impl fmt::Display for EvaluationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Marks => "MARKS",
            Self::Grade => "Grade",
        };

        write!(f, "{s}")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssessmentBody {
    pub term: Uuid,
    pub grade: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssessmentResponse {
    pub assessment_id: Uuid,

    pub term_id: Uuid,
    pub term_name: String,

    pub subject_id: Uuid,
    pub subject_name: String,
    pub subject_code: String,

    pub exam_id: Uuid,
    pub exam_display_name: String,

    pub exam_type_id: Uuid,
    pub exam_type_code: String,

    pub evaluation_type: EvaluationType,
    pub max_marks: Option<i32>,
}

#[tracing::instrument(name = "getting the assesment from db", skip(pool), 
    fields (
        grade_id= %query.grade,
        term_id= %query.term
))]
#[get("/assessment-scheme")]
pub async fn get_assessment_scheme(
    pool: web::Data<PgPool>,
    query: web::Query<AssessmentBody>,
) -> Result<HttpResponse, AppError> {
    // let rows = sqlx::query_as!(
    //     AssessmentResponse,
    //     r#"
    //     SELECT
    //         a.id           AS "assessment_id!",
    //
    //         t.id           AS "term_id!",
    //         t.name         AS "term_name!",
    //
    //         s.id           AS "subject_id!",
    //         s.name         AS "subject_name!",
    //         s.code         AS "subject_code!",
    //
    //         e.id           AS "exam_id!",
    //         e.display_name AS "exam_display_name!",
    //
    //         et.id          AS "exam_type_id!",
    //         et.code        AS "exam_type_code!",
    //
    //         a.evaluation_type AS "evaluation_type!",
    //         a.max_marks
    //     FROM assessment_scheme a
    //     JOIN exams e
    //         ON a.exam_id = e.id
    //     JOIN exam_types et
    //         ON e.exam_type_id = et.id
    //     JOIN terms t
    //         ON e.term_id = t.id
    //     JOIN subjects s
    //         ON a.subject_id = s.id
    //     WHERE a.grade_id = $1
    //       AND e.term_id = $2
    //     ORDER BY
    //         s.name,
    //         et.code
    //     "#,
    //     query.grade,
    //     query.term
    // )
    // .fetch_all(pool.get_ref())
    // .await
    // .map_err(|e| {
    //     tracing::error!(error=?e, "Error fetching assesment");
    //     AppError::Internal
    // })?;

    Ok(HttpResponse::Ok().finish())
}
