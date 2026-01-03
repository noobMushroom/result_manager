use crate::routes::domain::util::{contain_forbidden_characters, is_empty_or_whitespace};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Username(String);

impl Username {
    #[allow(dead_code)]
    pub fn parse(s: &str) -> Self {
        let is_empty_or_whitespace = is_empty_or_whitespace(s);
        let contain_forbidden_characters = contain_forbidden_characters(s);
        if contain_forbidden_characters || is_empty_or_whitespace {
            todo!()
        } else {
            Self(s.to_string())
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
        let username = Username::parse("some");
        assert_eq!("some", username.as_ref());
    }
}
