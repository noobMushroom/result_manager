use crate::domain::roles::Role;
use chrono::{Duration, Utc};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Claims {
    pub sub: Uuid,
    pub role: Role,
    pub exp: usize,
    pub issued_at: usize,
}

impl Claims {
    pub fn new(sub: Uuid, role: Role) -> Self {
        let exp = Utc::now()
            .checked_add_signed(Duration::days(14))
            .expect("Valid timestamp")
            .timestamp() as usize;
        Self {
            sub,
            role,
            exp,
            issued_at: Utc::now().timestamp() as usize,
        }
    }
}
