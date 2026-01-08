use crate::domain::grade::Grade;
use crate::domain::username::Username;
use chrono::NaiveDate;

pub struct NewStudent {
    pub name: Username,
    pub father_name: Username,
    pub grade: Grade,
    pub date_of_birth: NaiveDate,
    pub adm_no: i32,
}
