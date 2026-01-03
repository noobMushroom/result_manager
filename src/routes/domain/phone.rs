use crate::routes::domain::util::{contain_forbidden_characters, is_empty_or_whitespace};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Phone(String);

impl Phone {
    #[allow(dead_code)]
    pub fn parse(s: &str) -> Self {
        let is_empty_or_whitespace = is_empty_or_whitespace(s);
        let contain_forbidden_characters = contain_forbidden_characters(s);
        let is_too_long = s.len() != 10;

        if contain_forbidden_characters || is_empty_or_whitespace || is_too_long {
            todo!()
        } else {
            Self(s.to_string())
        }
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
        let phone = Phone::parse("0123456789");
        assert_eq!("0123456789", phone.as_ref());
    }
}
