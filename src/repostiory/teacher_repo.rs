use crate::{
    domain::{errors::DomainError, new_teacher::NewTeacher, phone::Phone},
    routes::teachers::TeacherDetails,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Repository pattern for teachers method
pub struct TeacherRepo {
    pub pool: PgPool,
}

impl TeacherRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    /// Inserts the new teachers to the db
    #[tracing::instrument(
        name = "saving the new teacher to the database",
        skip(new_teacher, self)
    )]
    pub async fn insert_teacher(&self, new_teacher: &NewTeacher) -> Result<(), DomainError> {
        match sqlx::query!(
            r#"
            INSERT INTO teachers(id, phone_no, name, subscribed_at, role)
            VALUES ($1, $2, $3, $4, $5)
        "#,
            Uuid::new_v4(),
            new_teacher.phone.as_ref(),
            new_teacher.name.as_ref(),
            Utc::now(),
            new_teacher.role.to_string()
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_error))
                if db_error.constraint() == Some("teachers_phone_no_key") =>
            {
                Err(DomainError::DuplicatePhoneNo)
            }

            _ => Err(DomainError::Internal),
        }
    }

    /// Gets the teacher from db
    #[tracing::instrument(name = "Getting user from db", skip(self, phone), fields(phone = %phone.as_ref()))]
    pub async fn get_teacher(&self, phone: &Phone) -> Result<TeacherDetails, DomainError> {
        let teacher_details = sqlx::query_as!(
            TeacherDetails,
            r#"
        SELECT id, phone_no, name, role
        FROM teachers
        WHERE phone_no = $1
        LIMIT 1
        "#,
            phone.as_ref()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!(error=?e, "error while getting getting the teacher details from db");
            DomainError::Internal
        })?;

        Ok(teacher_details)
    }

    /// Checks if teacher is present or not in the db
    #[tracing::instrument(name = "checking if the teacher is registered", skip(phone, self))]
    pub async fn check_teacher_exist(&self, phone: &Phone) -> Result<bool, DomainError> {
        let user = sqlx::query_scalar!(
            "SELECT 1 FROM teachers WHERE phone_no = $1 LIMIT 1",
            phone.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!(error=?e, "Error while checing if the user exist");
            DomainError::Internal
        })?
        .is_some();

        Ok(user)
    }
}
