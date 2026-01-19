use crate::domain::errors::DomainError;

pub mod get_assesment;
pub mod get_exam_types;
pub mod get_grades;
pub mod get_terms;

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub enum ExamTypes {
    UNIT_TEST,
    CLASS_PER,
    COPY_WORK,
    ACTIVITY,
    HALF_YEARLY,
    ORAL_PRAC,
    WRITT,
    COPY_NEATNESS,
    HOME_WORK,
    ANNUAL,
}

impl TryFrom<&str> for ExamTypes {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "UT" => Ok(Self::UNIT_TEST),
            "CLASS_PER" => Ok(Self::CLASS_PER),
            "COPY_WORK" => Ok(Self::COPY_WORK),
            "ACTIVITY" => Ok(Self::ACTIVITY),
            "HALF_YEARLY" => Ok(Self::HALF_YEARLY),
            "ORAL_PRAC" => Ok(Self::ORAL_PRAC),
            "WRITT" => Ok(Self::WRITT),
            "CN" => Ok(Self::COPY_NEATNESS),
            "HW" => Ok(Self::HOME_WORK),
            "ANNUAL" => Ok(Self::ANNUAL),
            _ => Err(DomainError::Internal),
        }
    }
}

impl From<String> for ExamTypes {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "UT" => Self::UNIT_TEST,
            "CLASS_PER" => Self::CLASS_PER,
            "COPY_WORK" => Self::COPY_WORK,
            "ACTIVITY" => Self::ACTIVITY,
            "HALF_YEARLY" => Self::HALF_YEARLY,
            "ORAL_PRAC" => Self::ORAL_PRAC,
            "WRITT" => Self::WRITT,
            "CN" => Self::COPY_NEATNESS,
            "HW" => Self::HOME_WORK,
            "ANNUAL" => Self::ANNUAL,
            _ => Self::ANNUAL,
        }
    }
}
