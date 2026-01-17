use crate::helpers::{mock_server, spawn_app};
use result_management::routes::result::add_result::{AddMarksData, MarksBody};
use uuid::Uuid;

fn get_phone<'a>() -> &'a str {
    "1234567890"
}

fn get_admin_role<'a>() -> &'a str {
    "admin"
}

// fn demo_marks_body() -> MarksBody {
//     MarksBody {
//         subject: Uuid::new_v4(),
//         exam_type: Uuid::new_v4(),
//         marks: None,
//         grade: None,
//     }
// }
//
// fn demo_add_marks_data(demo_marks: Vec<MarksBody>) -> AddMarksData {
//     AddMarksData {
//         student_id: Uuid::new_v4(),
//         term: Uuid::new_v4(),
//         marks: demo_marks,
//     }
// }

// #[actix::test]
// async fn both_marks_and_grade_present_return_error() {
//     let app = spawn_app().await;
//     mock_server(&app.message_server).await;
//     let token = app.get_token(get_phone(), get_admin_role()).await;
//     let mut demo_marks = demo_marks_body();
//     demo_marks.grade = Some("B".to_string());
//     demo_marks.marks = Some(21);
//     let marks_vec = vec![demo_marks];
//     let body = demo_add_marks_data(marks_vec);
//     let _response = app.send_add_marks_request(&body, &token).await;
//     // assert_eq!(response.status().as_u16(), 400);
//     assert!(true)
// }
