use actix_web::error::ErrorBadRequest;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};

#[derive(Debug)]
pub struct Engine {
    pub name: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
}

#[derive(Debug)]
pub struct Product {
    pub name: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

impl From<user_agent_parser::Product<'_>> for Product {
    fn from(product: user_agent_parser::Product) -> Self {
        Product {
            name: if let Some(name) = product.name {
                Some(format!("{}", name.into_owned()))
            } else {
                None
            },
            major: if let Some(major) = product.major {
                Some(format!("{}", major.into_owned()))
            } else {
                None
            },
            minor: if let Some(minor) = product.minor {
                Some(format!("{}", minor.into_owned()))
            } else {
                None
            },
            patch: if let Some(patch) = product.patch {
                Some(format!("{}", patch.into_owned()))
            } else {
                None
            },
        }
    }
}

#[derive(Debug)]
pub struct UserAgent {
    // pub product: Product
}

impl FromRequest for UserAgent {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        if let Some(ua) = req.headers().get("User-Agent") {
            println!("{:?}", ua);
            // use user_agent_parser::UserAgentParser;
            // let ua_parser = UserAgentParser::from_path("regexes.yaml").unwrap();
            // let ua = ua.to_str().unwrap();

            // // println!("{:?}", ua);

            // let product = ua_parser.parse_product(ua);
            // let engine = ua_parser.parse_engine(ua);
            // println!("{:#?}", engine);

            ok(UserAgent {
                // product: Product {
                //     name: if let Some(name) = product.name {
                //         Some(format!("{}", name.into_owned()))
                //     } else {
                //         None
                //     },
                //     major: if let Some(major) = product.major {
                //         Some(format!("{}", major.into_owned()))
                //     } else {
                //         None
                //     },
                //     minor: if let Some(minor) = product.minor {
                //         Some(format!("{}", minor.into_owned()))
                //     } else {
                //         None
                //     },
                //     patch: if let Some(patch) = product.patch {
                //         Some(format!("{}", patch.into_owned()))
                //     } else {
                //         None
                //     },
                // },
            })
        } else {
            err(ErrorBadRequest("no luck"))
        }
    }
}
