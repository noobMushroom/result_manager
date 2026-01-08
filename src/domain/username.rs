use crate::domain::errors::DomainError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Username(String);

pub fn contain_forbidden_characters(s: &str) -> bool {
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\'];
    s.chars()
        .any(|c| c.is_numeric() || forbidden_characters.contains(&c))
}

impl Username {
    pub fn parse(s: &str) -> Result<Self, DomainError> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let contain_forbidden_characters = contain_forbidden_characters(s);
        if contain_forbidden_characters || is_empty_or_whitespace {
            return Err(DomainError::InvalidName);
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_correct_name() {
        let username = Username::parse("some").unwrap();
        assert_eq!("some", username.as_ref());
    }

    #[test]
    fn return_error_in_incorrect() {
        let cases = ["some9", " ", "/hueho/"];

        for case in cases {
            let username = Username::parse(case);
            assert!(username.is_err(), "Case should be invalid {}", case);
        }
    }
}
