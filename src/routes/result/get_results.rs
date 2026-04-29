use crate::{
    errors::AppError,
    routes::academics::{Sections, get_grades::get_grade_info},
};
use actix_web::{HttpResponse, get, web};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetResultQuery {
    pub section: Option<Sections>,
}

// #[get("/get_results/{grade}")]
// pub async fn get_results(
//     pool: web::Data<PgPool>,
//     req: web::Path<String>,
//     query: web::Query<GetResultQuery>,
// ) -> Result<HttpResponse, AppError> {
//     let section: Option<&Sections> = query.section.as_ref();
//     let grade = req.into_inner();
//     let grade_id = get_grade_info(&pool, &grade)
//         .await?
//         .ok_or_else(|| AppError::BadRequest("Invaild Grade".into()))?;
//
//     todo!()
// }
//
// fn get_query<'a>(grade: Uuid, body: &'a GetResultQuery) -> QueryBuilder<'a, Postgres> {
//     let mut query = QueryBuilder::new(
//         "Select * from results JOIN students ON students.id= results.student_id WHERE grade = ",
//     );
//
//     query.push_bind(grade);
//
//     if let Some(section) = body.section {
//         query.push(" AND section = ");
//         query.push_bind(section.as_ref());
//     }
//
//     todo!()
// }
