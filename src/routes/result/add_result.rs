use std::fmt;

use crate::{domain::errors::DomainError, errors::AppError};
use actix_web::{HttpResponse, post, web};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum Status {
    PRESENT,
    ABSENT,
    MEDICAL,
}

impl TryFrom<&str> for Status {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "PRESENT" => Ok(Self::PRESENT),
            "MEDICAL" => Ok(Self::MEDICAL),
            "ABSENT" => Ok(Self::ABSENT),
            _ => Err(DomainError::InvalidRole),
        }
    }
}

impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        match self {
            Status::PRESENT => "PRESENT",
            Status::ABSENT => "ABSENT",
            Status::MEDICAL => "MEDICAL",
        }
    }
}

impl TryFrom<String> for Status {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PRESENT => "PRESENT",
            Self::ABSENT => "ABSENT",
            Self::MEDICAL => "MEDICAL",
        };

        write!(f, "{s}")
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct AddMarksData {
    pub student_id: Uuid,
    pub marks: Vec<MarksBody>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct MarksBody {
    pub assessment_id: Uuid,
    pub marks: Option<i32>,
    pub grade: Option<String>,
    pub status: Option<Status>,
}

impl MarksBody {
    pub fn validate(&self) -> Result<(), DomainError> {
        let status = self.status.clone().unwrap_or(Status::PRESENT);

        match status {
            Status::PRESENT => {
                if self.marks.is_some() && self.grade.is_some() {
                    return Err(DomainError::BadRequest(
                        "Both marks and grade provided".into(),
                    ));
                }
            }

            Status::MEDICAL | Status::ABSENT => {
                if self.marks.is_some() || self.grade.is_some() {
                    return Err(DomainError::BadRequest(
                        "Marks/grade not allowed for ABSENT or MEDICAL".into(),
                    ));
                }
            }
        }

        Ok(())
    }
}

struct InsertResultBody<'a> {
    student_id: Uuid,
    marks: &'a MarksBody,
}

#[tracing::instrument(name = "Adding marks to the database", skip(pool))]
#[post["/add_marks"]]
pub async fn add_result(
    pool: web::Data<PgPool>,
    body: web::Json<AddMarksData>,
) -> Result<HttpResponse, AppError> {
    for marks in body.marks.iter() {
        marks.validate()?;
        let insert_body = InsertResultBody {
            student_id: body.student_id,
            marks,
        };

        insert_result(&pool, &insert_body).await?;
    }

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Inserting result", skip(pool, body))]
async fn insert_result<'a>(pool: &PgPool, body: &InsertResultBody<'a>) -> Result<(), DomainError> {
    // let status = body.marks.status.clone().unwrap_or(Status::PRESENT);
    // match status {
    //     Status::ABSENT | Status::MEDICAL => {
    //         sqlx::query!(
    //             r#"
    //             INSERT INTO results (
    //                 student_id,
    //                 assessment_id,
    //                 marks_obtained,
    //                 grade,
    //                 result_status
    //             )
    //             VALUES ($1, $2, NULL, NULL, $3)
    //             ON CONFLICT (student_id, assessment_id)
    //             DO UPDATE SET
    //                 marks_obtained = NULL,
    //                 grade = NULL,
    //                 result_status = EXCLUDED.result_status,
    //                 updated_at = now()
    //             "#,
    //             body.student_id,
    //             body.marks.assessment_id,
    //             status.as_ref(),
    //         )
    //         .execute(pool)
    //         .await
    //         .map_err(|e| {
    //             tracing::error!(error=?e, "failed to upsert absent/medical result");
    //             DomainError::Internal
    //         })?;
    //     }
    //
    //     Status::PRESENT => {
    //         if body.marks.marks.is_none() && body.marks.grade.is_none() {
    //             // explicit clear
    //             sqlx::query!(
    //                 r#"
    //                 DELETE FROM results
    //                 WHERE student_id = $1
    //                   AND assessment_id = $2
    //                 "#,
    //                 body.student_id,
    //                 body.marks.assessment_id,
    //             )
    //             .execute(pool)
    //             .await
    //             .map_err(|e| {
    //                 tracing::error!(error=?e, "failed to delete result");
    //                 DomainError::Internal
    //             })?;
    //         } else {
    //             sqlx::query!(
    //                 r#"
    //                 INSERT INTO results (
    //                     student_id,
    //                     assessment_id,
    //                     marks_obtained,
    //                     grade,
    //                     result_status
    //                 )
    //                 VALUES ($1, $2, $3, $4, 'PRESENT')
    //                 ON CONFLICT (student_id, assessment_id)
    //                 DO UPDATE SET
    //                     marks_obtained = EXCLUDED.marks_obtained,
    //                     grade = EXCLUDED.grade,
    //                     result_status = 'PRESENT',
    //                     updated_at = now()
    //                 "#,
    //                 body.student_id,
    //                 body.marks.assessment_id,
    //                 body.marks.marks,
    //                 body.marks.grade,
    //             )
    //             .execute(pool)
    //             .await
    //             .map_err(|e| {
    //                 tracing::error!(error=?e, "failed to upsert present result");
    //                 DomainError::Internal
    //             })?;
    //         }
    //     }
    // }

    Ok(())
}
