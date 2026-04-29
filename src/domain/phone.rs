use crate::domain::errors::DomainError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Phone(String);

impl Phone {
    pub fn parse(s: &str) -> Result<Self, DomainError> {
        if s.trim().is_empty() {
            return Err(DomainError::InvalidPhone);
        }

        if s.chars().any(|c| !c.is_numeric()) || s.len() != 10 {
            return Err(DomainError::InvalidPhone);
        }

        Ok(Self(s.to_string()))
    }
}

impl AsRef<str> for Phone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let phone = Phone::parse("0123456789").unwrap();
        assert_eq!("0123456789", phone.as_ref());
    }

    #[test]
    fn test_parse_invalid() {
        let cases = [" ", "auha", "123456789"];
        for case in cases {
            let phone = Phone::parse(case);
            assert!(phone.is_err(), "number should be invalid {}", case);
        }
    }
}
