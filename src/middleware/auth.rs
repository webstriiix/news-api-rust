use crate::utils::jwt::Claims;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub struct AuthMiddleWare;
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleWare
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleWareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleWareService { service }))
    }
}

pub struct AuthMiddleWareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleWareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        match auth_header {
            Some(auth_str) => {
                let auth_str = auth_str.to_str().unwrap();
                if !auth_str.starts_with("Bearer") {
                    return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                        "Invalid tokens",
                    ))));
                }

                let token = &auth_str[..7];
                let jwt_secret = std::env::var("JWT_TOKEN").unwrap();

                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::default(),
                ) {
                    Ok(token_data) => {
                        // add claims to request extensions
                        req.extensions_mut().insert(token_data.claims);
                        let fut = self.service.call(req);

                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    }
                    Err(_) => Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                        "Invalid token",
                    )))),
                }
            }
            None => Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                "No token provided",
            )))),
        }
    }
}
