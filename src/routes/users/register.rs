use crate::domain::errors::DomainError;
use crate::domain::username::Username;
use crate::domain::{new_teacher::NewTeacher, phone::Phone};
use crate::errors::AppError;
use actix_web::{HttpResponse, post, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct UserData {
    name: String,
    phone: String,
    role: String,
}

impl TryFrom<UserData> for NewTeacher {
    type Error = DomainError;

    fn try_from(value: UserData) -> Result<Self, Self::Error> {
        let name = Username::parse(&value.name)?;
        let phone = Phone::parse(&value.phone)?;
        let role = value.role.as_str().try_into()?;

        Ok(Self { name, phone, role })
    }
}

#[tracing::instrument(
    name = "adding a new teacher",
    skip(json, connection),
    fields(
        name = %json.name,
        phone = %json.phone
    )
)]
#[post("/add_teacher")]
pub async fn add_teacher(
    json: web::Json<UserData>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let new_teacher: NewTeacher = json.into_inner().try_into()?;
    insert_teacher(&connection, &new_teacher).await?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "saving the new teacher to the database",
    skip(new_teacher, pool)
)]
async fn insert_teacher(pool: &PgPool, new_teacher: &NewTeacher) -> Result<(), DomainError> {
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
    .execute(pool)
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
