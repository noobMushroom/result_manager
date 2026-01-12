use crate::domain::errors::DomainError;
use std::fmt;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Teacher,
}

impl TryFrom<&str> for Role {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "admin" => Ok(Self::Admin),
            "teacher" => Ok(Self::Teacher),
            _ => Err(DomainError::InvalidRole),
        }
    }
}

impl TryFrom<String> for Role {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Admin => "admin",
            Self::Teacher => "teacher",
        };

        write!(f, "{s}")
    }
}
