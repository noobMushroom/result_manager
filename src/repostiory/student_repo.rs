use chrono::NaiveDate;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{
    cache::app_cache::AppCache,
    domain::{errors::DomainError, grade::Grade, new_student::NewStudent, username::Username},
    routes::{academics::Sections, students::get_student::SearchStudentsQuery},
};

pub struct StudentRepo {
    pool: PgPool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, FromRow)]
pub struct GetStudentsResponse {
    pub id: Uuid,
    pub name: String,
    pub father_name: String,
    pub grade_name: String,
    pub admission_no: i32,
    pub date_of_birth: NaiveDate,
}

impl StudentRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    /// Insert's the student to the db
    #[tracing::instrument(name = "saving the new student to the database", skip(student, self))]
    pub async fn insert_student(
        &self,
        student: NewStudent,
        app_cache: &AppCache,
    ) -> Result<(), DomainError> {
        let grade_id = app_cache
            .get_grade_id(&student.grade)
            .ok_or_else(|| DomainError::Internal)?;

        let sections_id = student
            .section
            .map(|section| {
                app_cache
                    .get_section_id(&section)
                    .ok_or_else(|| DomainError::Internal)
            })
            .transpose()?;
        match sqlx::query!(
            r#"
                INSERT INTO students (grade_id, name, father_name, admission_no, date_of_birth, section_id)
                VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            grade_id,
            student.name.as_ref(),
            student.father_name.as_ref(),
            student.adm_no,
            student.date_of_birth,
            sections_id
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_error))
                if db_error.constraint() == Some("students_admission_no_key") =>
            {
                Err(DomainError::DuplicateAdmissionNo)
            }

            _ => Err(DomainError::Internal),
        }
    }

    pub async fn get_students(
        &self,
        query: &SearchStudentsQuery,
        app_cache: &AppCache,
    ) -> Result<Vec<GetStudentsResponse>, DomainError> {
        let mut qb = sqlx::QueryBuilder::new("SELECT * FROM students");

        if query.has_filters() {
            let mut separated = qb.separated(" AND ");
            separated.push_unseparated(" WHERE ");

            if let Some(grade) = &query.grade {
                let grade = Grade::parse(grade)?;
                let grade_id = app_cache
                    .get_grade_id(&grade)
                    .ok_or(DomainError::Internal)?;
                separated.push("grade_id = ");
                separated.push_bind_unseparated(grade_id);
            }

            if let Some(name) = &query.name {
                let name = Username::parse(name)?;
                separated.push("name ILIKE ");
                separated.push_bind_unseparated(format!("%{}%", name.as_ref()));
            }

            if let Some(adm_no) = &query.admission_no {
                separated.push("admission_no = ");
                separated.push_bind_unseparated(adm_no);
            }

            if let Some(section) = &query.section {
                let section: Sections = section.as_str().try_into()?;
                let section_id = app_cache
                    .get_section_id(&section)
                    .ok_or(DomainError::Internal)?;
                separated.push("section_id = ");
                separated.push_bind_unseparated(section_id);
            }
        }

        let limit = query.limit.unwrap_or(20);
        let offset = query.offset.unwrap_or(0);

        qb.push(" LIMIT ");
        qb.push_bind(limit);
        qb.push(" OFFSET ");
        qb.push_bind(offset);

        let students: Vec<GetStudentsResponse> = qb
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Error searching students");
                DomainError::Internal
            })?;

        Ok(students)
    }
}
