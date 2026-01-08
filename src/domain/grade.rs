use crate::domain::errors::DomainError;

pub struct Grade(String);

impl Grade {
    pub fn parse(s: &str) -> Result<Self, DomainError> {
        let s = s.trim();

        if s.is_empty() {
            return Err(DomainError::InvalidGrade);
        }
        let normalized = s.to_uppercase();

        let grades = [
            "NURSERY", "LKG", "UKG", "1", "2", "3", "4", "5", "6", "7", "8",
        ];

        if !grades.contains(&normalized.as_str()) {
            return Err(DomainError::InvalidGrade);
        }

        Ok(Self(s.to_string()))
    }
}

impl AsRef<str> for Grade {
    fn as_ref(&self) -> &str {
        &self.0
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
        let cases = ["", " ", "KG", "0", "9", "Class 1", "Ten"];

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
