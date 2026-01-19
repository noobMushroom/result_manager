use actix_web::{HttpResponse, post, web};
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{errors::DomainError, grade::Grade, new_student::NewStudent, username::Username},
    errors::AppError,
    routes::academics::Sections,
};

#[derive(serde::Deserialize)]
pub struct StudentData {
    pub name: String,
    pub date_of_birth: String,
    pub admission_no: i32,
    pub father_name: String,
    pub grade: String,
    pub section: Option<Sections>,
}

impl TryFrom<StudentData> for NewStudent {
    type Error = DomainError;
    fn try_from(value: StudentData) -> Result<Self, Self::Error> {
        let name = Username::parse(&value.name)?;
        let father_name = Username::parse(&value.father_name)?;
        let grade = Grade::parse(&value.grade)?;
        let date_of_birth = NaiveDate::parse_from_str(&value.date_of_birth, "%d-%m-%Y")
            .map_err(|_| DomainError::InvalidDateOfBirth)?;

        Ok(Self {
            adm_no: value.admission_no,
            name,
            father_name,
            grade,
            date_of_birth,
            section: value.section,
        })
    }
}

#[tracing::instrument(
    name = "adding a new student",
    skip(json, connection),
    fields(
        name = %json.name,
        father_name = %json.father_name,
        date_of_birth = %json.date_of_birth,
        admission_no = %json.admission_no,
        grade = %json.grade,
    )
)]
#[post("/add_student")]
pub async fn register_student(
    json: web::Json<StudentData>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let new_student: NewStudent = json.into_inner().try_into()?;
    insert_student(&connection, &new_student).await?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "saving the new student to the database", skip(student, pool))]
pub async fn insert_student(pool: &PgPool, student: &NewStudent) -> Result<(), DomainError> {
    let grade = get_grade_uuid(pool, student.grade.as_ref()).await?;
    match sqlx::query!(
        r#"
            INSERT INTO students (id, grade_id, name, father_name, admission_no, date_of_birth, section)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        Uuid::new_v4(),
        grade,
        student.name.as_ref(),
        student.father_name.as_ref(),
        student.adm_no,
        student.date_of_birth,
        student.section.as_ref().map(|s| s.as_ref()),
    )
    .execute(pool)
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

// Function to get grade uuid from db
#[tracing::instrument(name = "getting the grade uuid from db", skip(grade, pool))]
pub async fn get_grade_uuid(pool: &PgPool, grade: &str) -> Result<Uuid, DomainError> {
    let uuid = sqlx::query!(
        r#"
            SELECT id FROM grades WHERE name= $1
        "#,
        grade
    )
    .fetch_one(pool)
    .await;

    match uuid {
        Ok(uuid) => Ok(uuid.id),
        Err(_) => Err(DomainError::InvalidGrade),
    }
}
