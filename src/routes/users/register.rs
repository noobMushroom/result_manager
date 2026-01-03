use crate::routes::domain::phone::Phone;
use crate::routes::domain::username::Username;
use actix_web::{HttpResponse, Responder, post, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UserData {
    pub username: Username,
    pub phone: Phone,
}

#[tracing::instrument(
    name = "adding a new subscriber",
    skip(json, connection),
    fields(
        name = %json.username.as_ref(),
        phone = %json.phone.as_ref()
    )
)]
#[post("/subscribe")]
pub async fn subscribe(json: web::Json<UserData>, connection: web::Data<PgPool>) -> impl Responder {
    match insert_subscriber(&connection, &json).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "saving the new subscriber to the database", skip(json, pool))]
pub async fn insert_subscriber(pool: &PgPool, json: &UserData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, name, phone_no, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        json.username.as_ref(),
        json.phone.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query {}", e);
        e
    })?;
    Ok(())
}
