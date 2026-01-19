use result_management::routes::{
    academics::get_assesment::{AssessmentBody, AssessmentResponse},
    result::add_result::{AddMarksData, MarksBody},
};
use uuid::Uuid;

use crate::{add_students::AddStudentdBody, helpers::spawn_app};

fn get_assessment_body(term: Uuid, grade: Uuid) -> AssessmentBody {
    AssessmentBody { term, grade }
}

struct SubjectInput {
    code: String,
    marks: Option<i32>,
    grade: Option<String>,
    exam_type_code: String,
}

fn get_json_body(
    assessment_scheme: &[AssessmentResponse],
    student_id: Uuid,
    subject_inputs: &[SubjectInput],
) -> AddMarksData {
    let marks = subject_inputs
        .iter()
        .map(|input| {
            let assessment = assessment_scheme
                .iter()
                .find(|a| a.subject_code == input.code && a.exam_type_code == input.exam_type_code)
                .unwrap_or_else(|| panic!("assessment not found for subject {}", input.code));

            MarksBody {
                assessment_id: assessment.assessment_id,
                marks: input.marks,
                grade: input.grade.clone(),
                status: None,
            }
        })
        .collect();

    AddMarksData { student_id, marks }
}
fn get_subject_body(body: &[(&str, Option<i32>, Option<String>, &str)]) -> Vec<SubjectInput> {
    body.into_iter()
        .map(|input| SubjectInput {
            code: input.0.to_string(),
            marks: input.1,
            grade: input.2.clone(),
            exam_type_code: input.3.to_string(),
        })
        .collect()
}

#[actix::test]
async fn add_results_for_valid_data() {
    let app = spawn_app().await;

    let grade = app.get_grade(&app.test_user.token, "8").await;
    let term = app.get_term(&app.test_user.token, "Second Term").await;
    let assessment_body = get_assessment_body(term.id, grade.id);
    let assesment_scheme = app.get_assesment_scheme(&assessment_body).await;
    let student_body = AddStudentdBody::new(
        "Raj     KumaRR",
        "12-12-2002",
        333,
        "Kamal Hasan",
        &grade.name,
    );
    let student = app.add_student_to_db(&student_body, grade.id).await;

    let subject_input_body = [
        ("MATHS", Some(50), None, "ANNUAL"),
        ("MATHS", Some(5), None, "CLASS_PER"),
        ("COMP", Some(50), None, "ANNUAL"),
        ("COMP", Some(6), None, "COPY_WORK"),
        ("MATHS", Some(8), None, "UT"),
        ("COMP", Some(8), None, "UT"),
    ];

    let subject_input = get_subject_body(&subject_input_body);
    let body = get_json_body(&assesment_scheme, student, &subject_input);
    let response = app
        .send_add_marks_request(&body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT *  FROM results")
        .fetch_all(&app.db_pool)
        .await
        .expect("failed to fetch new subscription.");

    assert_eq!(saved.len(), 6);
}

#[actix::test]
async fn return_bad_request_if_both_grade_and_marks_added() {
    let app = spawn_app().await;

    let grade = app.get_grade(&app.test_user.token, "8").await;
    let term = app.get_term(&app.test_user.token, "Second Term").await;
    let assessment_body = get_assessment_body(term.id, grade.id);
    let assesment_scheme = app.get_assesment_scheme(&assessment_body).await;
    let student_body = AddStudentdBody::new(
        "Raj     KumaRR",
        "12-12-2002",
        333,
        "Kamal Hasan",
        &grade.name,
    );
    let student = app.add_student_to_db(&student_body, grade.id).await;

    let subject_input_body = [("MATHS", Some(32), Some("b".to_string()), "ANNUAL")];

    let subject_input = get_subject_body(&subject_input_body);

    let body = get_json_body(&assesment_scheme, student, &subject_input);

    let response = app
        .send_add_marks_request(&body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 400);
}

#[actix::test]
async fn sending_null_null_deletest_the_result() {
    let app = spawn_app().await;

    let grade = app.get_grade(&app.test_user.token, "8").await;
    let term = app.get_term(&app.test_user.token, "Second Term").await;
    let assessment_body = get_assessment_body(term.id, grade.id);
    let assesment_scheme = app.get_assesment_scheme(&assessment_body).await;
    let student_body = AddStudentdBody::new(
        "Raj     KumaRR",
        "12-12-2002",
        333,
        "Kamal Hasan",
        &grade.name,
    );
    let student = app.add_student_to_db(&student_body, grade.id).await;

    let subject_input_body = [("MATHS", Some(50), None, "ANNUAL")];

    let subject_input = get_subject_body(&subject_input_body);

    let body = get_json_body(&assesment_scheme, student, &subject_input);
    let response = app
        .send_add_marks_request(&body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let saved =
        sqlx::query!("SELECT id, student_id, assessment_id, marks_obtained, grade  FROM results")
            .fetch_one(&app.db_pool)
            .await
            .expect("failed to fetch new subscription.");

    assert_eq!(saved.student_id, student);
    assert_eq!(saved.marks_obtained, Some(50));

    let subject_input_body = [("MATHS", None, None, "ANNUAL")];

    let subject_input = get_subject_body(&subject_input_body);

    let body = get_json_body(&assesment_scheme, student, &subject_input);
    let response = app
        .send_add_marks_request(&body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let saved =
        sqlx::query!("SELECT id, student_id, assessment_id, marks_obtained, grade  FROM results")
            .fetch_optional(&app.db_pool)
            .await
            .expect("failed to fetch new subscription.");

    assert!(saved.is_none())
}

#[actix::test]
async fn resending_the_data_should_update_marks() {
    let app = spawn_app().await;

    let grade = app.get_grade(&app.test_user.token, "8").await;
    let term = app.get_term(&app.test_user.token, "Second Term").await;
    let assessment_body = get_assessment_body(term.id, grade.id);
    let assesment_scheme = app.get_assesment_scheme(&assessment_body).await;
    let student_body = AddStudentdBody::new(
        "Raj     KumaRR",
        "12-12-2002",
        333,
        "Kamal Hasan",
        &grade.name,
    );
    let student = app.add_student_to_db(&student_body, grade.id).await;

    let subject_input_body = [("MATHS", Some(50), None, "ANNUAL")];

    let subject_input = get_subject_body(&subject_input_body);

    let body = get_json_body(&assesment_scheme, student, &subject_input);
    let response = app
        .send_add_marks_request(&body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let saved =
        sqlx::query!("SELECT id, student_id, assessment_id, marks_obtained, grade  FROM results")
            .fetch_one(&app.db_pool)
            .await
            .expect("failed to fetch new subscription.");

    assert_eq!(saved.student_id, student);
    assert_eq!(saved.marks_obtained, Some(30));

    let subject_input_body = [("MATHS", None, None, "ANNUAL")];

    let subject_input = get_subject_body(&subject_input_body);

    let body = get_json_body(&assesment_scheme, student, &subject_input);
    let response = app
        .send_add_marks_request(&body, &app.test_user.token)
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let saved =
        sqlx::query!("SELECT id, student_id, assessment_id, marks_obtained, grade  FROM results")
            .fetch_one(&app.db_pool)
            .await
            .expect("failed to fetch new subscription.");

    assert_eq!(saved.student_id, student);
    assert_eq!(saved.marks_obtained, Some(30));
}
