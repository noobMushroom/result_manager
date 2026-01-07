use crate::error::UserError;

pub struct Grade(String);

impl Grade {
    pub fn parse(s: &str) -> Result<Self, UserError> {
        let s = s.trim();

        if s.is_empty() {
            return Err(UserError::InvalidGrade);
        }

        let grades = [
            "Nursery", "LKG", "UKG", "1", "2", "3", "4", "5", "6", "7", "8",
        ];

        if !grades.contains(&s) {
            return Err(UserError::InvalidGrade);
        }

        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_grades_are_accepted() {
        let cases = [
            "Nursery", "LKG", "UKG", "1", "2", "3", "4", "5", "6", "7", "8",
        ];

        for grade in cases {
            let parsed = Grade::parse(grade);
            assert!(parsed.is_ok(), "grade `{}` should be valid", grade);
        }
    }

    #[test]
    fn invalid_grades_are_rejected() {
        let cases = [
            "", " ", "nursery", // wrong case
            "KG", "0", "9", "Class 1", "Ten",
        ];

        for grade in cases {
            let parsed = Grade::parse(grade);
            assert!(parsed.is_err(), "grade `{}` should be invalid", grade);
        }
    }

    #[test]
    fn trims_whitespace() {
        let parsed = Grade::parse(" 6 ");
        assert!(parsed.is_ok());
    }
}
