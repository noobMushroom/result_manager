use crate::domain::{phone::Phone, roles::Role, username::Username};

pub struct NewTeacher {
    pub name: Username,
    pub phone: Phone,
    pub role: Role,
}
