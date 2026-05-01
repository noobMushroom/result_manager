use uuid::Uuid;

pub mod login;
pub mod otp;
pub mod register;


pub struct TeacherDetails {
    pub id: Uuid,
    pub role: String,
    pub phone_no: String,
    pub name: String
}

