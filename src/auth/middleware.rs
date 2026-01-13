use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use secrecy::SecretString;

use crate::auth::jwt::verify_jwt;

pub async fn jwt_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let token: String = match req.headers().get("authorization") {
        Some(token) => token.to_str().expect("failed to convert").to_string(),
        None => return Err(actix_web::error::ErrorUnauthorized("No token")),
    };

    let token = token
        .strip_prefix("Bearer ")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Wrong token"))?;

    let secret = req
        .app_data::<actix_web::web::Data<SecretString>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Internal Error"))?;

    let claims = verify_jwt(token, secret)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Wrong token"))?;

    req.extensions_mut().insert(claims);
    next.call(req).await
}
