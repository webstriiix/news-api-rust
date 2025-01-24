use crate::utils::jwt::verify_token;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures::future::{ready, LocalBoxFuture, Ready};

// Middleware to authenticate requests using JWT
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
        // Extract Authorization header
        let auth_header = req.headers().get("Authorization");

        match auth_header {
            Some(auth_str) => {
                let auth_str = auth_str.to_str().unwrap_or("");
                if !auth_str.starts_with("Bearer ") {
                    return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                        "Invalid token format",
                    ))));
                }

                let token = auth_str.trim_start_matches("Bearer ");

                // Verify the JWT token
                match verify_token(token) {
                    Ok(claims) => {
                        // Store claims in request extensions
                        req.extensions_mut().insert(claims);
                        let fut = self.service.call(req);

                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    }
                    Err(_) => Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                        "Invalid or expired token",
                    )))),
                }
            }
            None => Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                "No token provided",
            )))),
        }
    }
}
