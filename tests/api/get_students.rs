use result_management::routes::students::get_student::GetStudentsResponse;

use crate::{
    add_students::AddStudentdBody,
    helpers::{mock_server, spawn_app},
};

fn get_data() -> Vec<AddStudentdBody> {
    let student1 = AddStudentdBody::new("Raju", "12-12-2025", 111, "Raju dad", "LKG");
    let student2 = AddStudentdBody::new("Some", "12-12-2025", 112, "Some dad", "1");
    let student3 = AddStudentdBody::new("Priya", "12-12-2025", 113, "Priya dad", "LKG");
    let student4 = AddStudentdBody::new("Lost cause", "12-12-2025", 114, "Lost dad", "2");
    let student5 = AddStudentdBody::new("Angoor", "12-12-2025", 115, "Angoor dad", "2");
    let student6 = AddStudentdBody::new("Snjay dutt", "12-12-2025", 116, "Sanjay dad", "5");
    let student7 = AddStudentdBody::new("Raju", "12-12-2025", 117, "Raju dad", "7");

    vec![
        student1, student2, student3, student4, student5, student6, student7,
    ]
}

#[actix::test]
async fn return_all_students() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let token = app.get_token("1234567890", "admin").await;
    let dummy_data = get_data();

    for body in dummy_data {
        app.add_student(&body, &token).await;
    }

    // let grade_id = app.get_grade(&token, "LKG").await;
    let response = app.get_students("lkg", &token).await;
    assert_eq!(response.status().as_u16(), 200);
    let body = response
        .json::<Vec<GetStudentsResponse>>()
        .await
        .expect("Failed to convert to the body");
    assert!(!body.is_empty());
    assert_eq!(body.len(), 2);
}

#[actix::test]
async fn return_all_students_despite_name_case() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let token = app.get_token("1234567890", "admin").await;
    let dummy_data = get_data();

    for body in dummy_data {
        app.add_student(&body, &token).await;
    }

    // let grade_id = app.get_grade(&token, "LKG").await;
    let response = app.get_students("lKg", &token).await;
    assert_eq!(response.status().as_u16(), 200);
    let body = response
        .json::<Vec<GetStudentsResponse>>()
        .await
        .expect("Failed to convert to the body");
    assert!(!body.is_empty());
    assert_eq!(body.len(), 2);
}

#[actix::test]
async fn return_empty_array_if_no_student() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let token = app.get_token("1234567890", "admin").await;
    let dummy_data = get_data();

    for body in dummy_data {
        app.add_student(&body, &token).await;
    }

    // let grade_id = app.get_grade(&token, "LKG").await;
    let response = app.get_students("3", &token).await;
    assert_eq!(response.status().as_u16(), 200);
    let body = response
        .json::<Vec<GetStudentsResponse>>()
        .await
        .expect("Failed to convert to the body");
    assert!(body.is_empty());
}

#[actix::test]
async fn return_error_if_invalid_name() {
    let app = spawn_app().await;
    mock_server(&app.message_server).await;
    let token = app.get_token("1234567890", "admin").await;
    let dummy_data = get_data();

    for body in dummy_data {
        app.add_student(&body, &token).await;
    }

    // let grade_id = app.get_grade(&token, "LKG").await;
    let response = app.get_students("invalid", &token).await;
    assert_eq!(response.status().as_u16(), 400);
}

// #[actix::test]
// async fn search_student_by_name() {
//     let app = spawn_app().await;
//     mock_server(&app.message_server).await;
//     let token = app.get_token("1234567890", "admin").await;
//     let dummy_data = get_data();
//
//     for body in dummy_data {
//         app.add_student(&body, &token).await;
//     }
//
//     let grade_id = app.get_grade(&token, "LKG").await;
//     let response = app.get_students(&grade_id.id, &token).await;
//     assert_eq!(response.status().as_u16(), 200);
//     let body = response
//         .json::<Vec<GetStudentsResponse>>()
//         .await
//         .expect("Failed to convert to the body");
//     assert!(!body.is_empty());
//     assert_eq!(body.len(), 2);
// }
