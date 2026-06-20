use actix_web::{HttpResponse, post, web};
use chrono::NaiveDate;

use crate::{
    cache::app_cache::AppCache,
    domain::{errors::DomainError, grade::Grade, new_student::NewStudent, username::Username},
    errors::AppError,
    repostiory::student_repo::StudentRepo,
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


/// Endpoint to add students in the db
#[tracing::instrument(
    name = "adding a new student",
    skip(json, app_cache, student_repo),
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
    app_cache: web::Data<AppCache>,
    student_repo: web::Data<StudentRepo>,
) -> Result<HttpResponse, AppError> {
    let new_student: NewStudent = json.into_inner().try_into()?;
    student_repo.insert_student(new_student, &app_cache).await?;
    Ok(HttpResponse::Ok().finish())
}
