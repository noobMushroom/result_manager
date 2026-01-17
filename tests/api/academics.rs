use result_management::routes::academics::{
    get_assesment::AssessmentResponse, get_grades::GradeBodyResponse, get_terms::TermsResponse,
};

use crate::helpers::{mock_server, spawn_app};

#[actix::test]
async fn get_all_grades() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let token = app.get_token("1234567890", "admin").await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/academics/grades", app.address))
        .bearer_auth(&token)
        .send()
        .await
        .expect("failed to send request");

    assert_eq!(response.status().as_u16(), 200);

    let grades: Vec<GradeBodyResponse> = response.json().await.expect("falied to convert to json");

    assert_eq!(grades[0].name, "NURSERY");
}

#[actix::test]
async fn get_all_terms() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let token = app.get_token("1234567890", "admin").await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/academics/terms", app.address))
        .bearer_auth(&token)
        .send()
        .await
        .expect("failed to send request");

    assert_eq!(response.status().as_u16(), 200);

    let terms: Vec<TermsResponse> = response.json().await.expect("falied to convert to json");

    assert_eq!(terms.len(), 4);
}

#[actix::test]
async fn get_assessment_test() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let token = app.get_token("1234567890", "admin").await;
    let term = app.get_term(&token, "Half Yearly Term").await;
    let grade = app.get_grade(&token, "LKG").await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "{}/academics/assessment-scheme?grade={}&term={}",
            app.address, grade.id, term.id
        ))
        .bearer_auth(&token)
        .send()
        .await
        .expect("failed to send request");

    assert_eq!(response.status().as_u16(), 200);

    let terms: Vec<AssessmentResponse> = response.json().await.expect("falied to convert to json");
    assert_eq!(terms.len(), 19);
    assert!(!terms.is_empty());
}

#[actix::test]
async fn get_assessment_return_empty_if_no_term() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let token = app.get_token("1234567890", "admin").await;
    let term = app.get_term(&token, "First Term").await;
    let grade = app.get_grade(&token, "NURSERY").await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "{}/academics/assessment-scheme?grade={}&term={}",
            app.address, grade.id, term.id
        ))
        .bearer_auth(&token)
        .send()
        .await
        .expect("failed to send request");

    assert_eq!(response.status().as_u16(), 200);

    let terms: Vec<AssessmentResponse> = response.json().await.expect("falied to convert to json");

    assert!(terms.is_empty());
}
