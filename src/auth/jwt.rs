use crate::{auth::claims::Claims, domain::roles::Role};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode, encode};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

pub fn generate_jwt(
    id: Uuid,
    role: Role,
    secret: &SecretString,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(id, role);
    Ok(encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.expose_secret().as_bytes()),
    )
    .expect("failed to encode"))
}

pub fn verify_jwt(
    token: &str,
    secret: &SecretString,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.expose_secret().as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{EncodingKey, encode};
    use secrecy::ExposeSecret;

    use super::*;

    fn secret() -> SecretString {
        let secret = "super big secret";
        SecretString::new(secret.into())
    }

    fn generate_expired_token(uuid: Uuid, role: Role, secret: &str) -> String {
        let expired_time = Utc::now()
            .checked_sub_signed(Duration::days(14))
            .expect("valid timestamp")
            .timestamp() as usize;
        let claims = Claims {
            issued_at: expired_time,
            exp: expired_time,
            role: role,
            sub: uuid,
        };

        encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("failed to encode expired token")
    }

    #[test]
    fn expired_token_is_rejected() {
        let secret = secret();
        let uuid = Uuid::new_v4();

        let token = generate_expired_token(uuid, Role::Admin, &secret.expose_secret());
        let claims = verify_jwt(&token, &secret);

        assert!(claims.is_err());
    }

    #[test]
    fn verify_correct_token() {
        let secret = secret();
        let uuid = Uuid::new_v4();
        let token = generate_jwt(uuid, Role::Admin, &secret).unwrap();
        let claims = verify_jwt(&token, &secret);
        assert!(claims.is_ok());
        assert_eq!(claims.unwrap().sub, uuid);
    }

    #[test]
    fn error_wrong_token() {
        let secret = secret();
        let claims = verify_jwt("huashuashush", &secret);
        assert!(claims.is_err());
    }

    #[test]
    fn token_signed_with_different_secret_fails() {
        let correct_secret = secret();
        let wrong_secret = SecretString::new("evil-secret".into());

        let uuid = Uuid::new_v4();
        let token = generate_jwt(uuid, Role::Admin, &correct_secret).unwrap();

        let claims = verify_jwt(&token, &wrong_secret);
        assert!(claims.is_err());
    }

    #[test]
    fn tampered_token_is_rejected() {
        let secret = secret();
        let uuid = Uuid::new_v4();

        let mut token = generate_jwt(uuid, Role::Admin, &secret).unwrap();

        // flip a byte
        token.pop();
        token.push('x');

        let claims = verify_jwt(&token, &secret);
        assert!(claims.is_err());
    }

    #[test]
    fn role_is_preserved_in_claims() {
        let secret = secret();
        let uuid = Uuid::new_v4();

        let token = generate_jwt(uuid, Role::Teacher, &secret).unwrap();
        let claims = verify_jwt(&token, &secret).unwrap();

        assert_eq!(claims.role, Role::Teacher);
    }
}
