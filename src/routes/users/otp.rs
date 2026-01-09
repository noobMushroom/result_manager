use argon2::{
    Argon2,
    password_hash::{SaltString, rand_core::OsRng},
};
use rand::{Rng, rng};

pub fn generate_otp() -> String {
    rng().random_range(100000..=999999).to_string()
}

#[warn(dead_code)]
fn generate_hash(value: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let mut out = [0u8; 64];

    argon2
        .hash_password_into(value.as_bytes(), salt.as_str().as_bytes(), &mut out)
        .expect("failed to generate hash");

    String::from_utf8(out.to_vec()).expect("failed to convert to the string")
}

// #[tracing::instrument(name = "saving otp in the database", skip(pool, phone))]
// async fn insert_otp(pool: &PgPool, phone: &Phone) -> Result<(), DomainError> {}
