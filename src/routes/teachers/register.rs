use crate::domain::errors::DomainError;
use crate::domain::phone::Phone;
use crate::domain::username::Username;
use crate::repostiory::teacher_repo::TeacherRepo;
use crate::{domain::new_teacher::NewTeacher, errors::AppError};
use actix_web::{HttpResponse, post, web};

#[derive(serde::Deserialize)]
struct AddTeacherBody {
    name: String,
    phone: String,
    role: String,
}

impl TryFrom<AddTeacherBody> for NewTeacher {
    type Error = DomainError;
    fn try_from(value: AddTeacherBody) -> Result<Self, Self::Error> {
        let name = Username::parse(&value.name)?;
        let phone = Phone::parse(&value.phone)?;
        let role = value.role.try_into()?;
        Ok(Self { name, phone, role })
    }
}

/// Adds the new teacher to the database
#[tracing::instrument(
    name = "adding a new teacher",
    skip(new_teacher_info, connection),
    fields(
        name = %new_teacher_info.name,
        phone = %new_teacher_info.phone,
        role = %new_teacher_info.role
    )
)]
#[post("/add_teacher")]
pub async fn add_teacher(
    new_teacher_info: web::Json<AddTeacherBody>,
    connection: web::Data<TeacherRepo>,
) -> Result<HttpResponse, AppError> {
    let new_teacher: NewTeacher = new_teacher_info.into_inner().try_into()?;
    connection.insert_teacher(&new_teacher).await?;
    Ok(HttpResponse::Ok().finish())
}
