use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Either, Ready};

#[derive(Default)]
pub struct RedirectWWW;

impl<S, B> Transform<S> for RedirectWWW
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RedirectWWWService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectWWWService { service })
    }
}
pub struct RedirectWWWService<S> {
    service: S,
}

impl<S, B> Service for RedirectWWWService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let host = req.connection_info().host().to_string();
        let uri = req.uri().to_owned();

        match path.as_str() {
            "/agence" => {
                Either::Right(ok(req.into_response(
                    HttpResponse::MovedPermanently()
                        .header(
                            http::header::LOCATION,
                            "https://greenassembly.fr/agence-digitale-verte",
                        )
                        // .header(http::header::LOCATION, "https://localhost:8443/agence-digitale-verte")
                        .header(http::header::REFERER, format!("http://{}{}", host, uri))
                        .finish()
                        .into_body(),
                )))
            }
            _ => {
                if req.connection_info().host() == "www.greenassembly.fr" {
                    Either::Right(ok(req.into_response(
                        HttpResponse::MovedPermanently()
                            .header(
                                http::header::LOCATION,
                                format!("https://greenassembly.fr{}", path),
                            )
                            .header(http::header::REFERER, format!("https://{}{}", host, path))
                            .finish()
                            .into_body(),
                    )))
                } else {
                    Either::Left(self.service.call(req))
                }
            }
        }
    }
}
