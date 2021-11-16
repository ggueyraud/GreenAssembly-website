use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Either, Ready};

#[derive(Default)]
pub struct RedirectHTTPS {
    replacements: Vec<(String, String)>,
}

impl RedirectHTTPS {
    pub fn with_replacements(replacements: &[(String, String)]) -> Self {
        RedirectHTTPS {
            replacements: replacements.to_vec(),
        }
    }
}

impl<S, B> Transform<S> for RedirectHTTPS
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RedirectHTTPSService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectHTTPSService {
            service,
            replacements: self.replacements.clone(),
        })
    }
}
pub struct RedirectHTTPSService<S> {
    service: S,
    replacements: Vec<(String, String)>,
}

impl<S, B> Service for RedirectHTTPSService<S>
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

        println!("Path: {}", path);

        match path.as_str() {
            "/agence" => {
                println!("Ok");
                Either::Right(ok(req.into_response(
                    HttpResponse::MovedPermanently()
                         .header(http::header::LOCATION, "https://greenassembly.fr/agence-digitale-verte")
                        //.header(http::header::LOCATION, "https://localhost:8443/agence-digitale-verte")
                        .header(http::header::REFERER, format!("http://{}{}", host, uri))
                        .finish()
                        .into_body(),
                )))
            },
            _ => {
                println!("Pas ok");
                if req.connection_info().scheme() == "https" {
                    Either::Left(self.service.call(req))
                } else {
                    let mut url = format!("https://{}{}", host, uri);
                    for (s1, s2) in self.replacements.iter() {
                        url = url.replace(s1, s2);
                    }
                    Either::Right(ok(req.into_response(
                        HttpResponse::MovedPermanently()
                            .header(http::header::LOCATION, url)
                            .header(http::header::REFERER, format!("http://{}{}", host, uri))
                            .finish()
                            .into_body(),
                    )))
                }
            }
        }
    }
}
