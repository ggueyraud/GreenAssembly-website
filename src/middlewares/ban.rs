use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use futures::future::{ok, Either, Ready};

pub struct Ban {
    pub pool: sqlx::PgPool,
}

impl<S, B> Transform<S> for Ban
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = BanMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(BanMiddleware {
            service,
            pool: self.pool.clone(),
        })
    }
}

pub struct BanMiddleware<S> {
    service: S,
    pool: sqlx::PgPool,
}

impl<S, B> Service for BanMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let res = futures::executor::block_on(crate::services::ips_banned::get(
            &self.pool,
            req.connection_info().realip_remote_addr().unwrap(),
        ));

        println!(
            "realip: {:?} | addr: {:?}",
            req.connection_info().realip_remote_addr(),
            req.connection_info().remote_addr()
        );

        match res {
            Ok(ip_banned) => {
                Either::Right(ok(req.into_response(
                    HttpResponse::Forbidden()
                        .content_type("text/plain; charset=utf-8")
                        .body(format!("Votre adresse IP a été détecté comme intrusive, l'accès au site-web vous est interdis jusqu'au {}", ip_banned.date.format("%d/%m/%Y à %H:%M")))
                        .into_body()
                )))
            }
            _ => Either::Left(self.service.call(req)),
        }
    }
}
