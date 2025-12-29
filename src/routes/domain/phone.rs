use crate::routes::domain::util::{contain_forbidden_characters, is_empty_or_whitespace};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Phone(String);

impl Phone {
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