use crate::routes::domain::phone::Phone;
use crate::routes::domain::username::Username;
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UserData {
    pub username: Username,
    pub phone: Phone,
}

#[post("/subscribe")]
pub async fn subscribe(json: web::Json<UserData>, connection: web::Data<PgPool>) -> impl Responder {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, name, phone_no, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        json.username.as_ref(),
        json.phone.as_ref(),
        Utc::now()
    ).execute(connection.get_ref())
        .await
        .unwrap();
    HttpResponse::Ok()
}

