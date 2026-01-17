use crate::domain::errors::DomainError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Username(String);

pub fn captilize_first_letter(name: &str) -> String {
    name.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl Username {
    pub fn parse(s: &str) -> Result<Self, DomainError> {
        if s.trim().is_empty() {
            return Err(DomainError::InvalidName);
        }
        if s.chars()
            .any(|c| !c.is_alphabetic() && !c.is_ascii_whitespace())
        {
            return Err(DomainError::InvalidName);
        }

        let correctd = captilize_first_letter(s);

        Ok(Self(correctd.to_string()))
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
        assert_eq!("Some", username.as_ref());
    }

    #[test]
    fn parse_correct_name_long() {
        let username = Username::parse("long name").unwrap();
        assert_eq!("Long Name", username.as_ref());
    }

    #[test]
    fn return_error_name_long() {
        let username = Username::parse("long n8ame");
        assert!(username.is_err());
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
