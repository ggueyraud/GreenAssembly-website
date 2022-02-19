use actix_web::error::ErrorBadRequest;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};

#[derive(Debug)]
pub struct OS {
    pub name: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

impl From<user_agent_parser::OS<'_>> for OS {
    fn from(os: user_agent_parser::OS) -> Self {
        OS {
            name: os.name.map(|name| name.into_owned()),
            major: os.major.map(|major| major.into_owned()),
            minor: os.minor.map(|minor| minor.into_owned()),
            patch: os.patch.map(|patch| patch.into_owned()),
            patch_minor: os
                .patch_minor
                .map(|patch_minor| patch_minor.into_owned()),
        }
    }
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
            name: product.name.map(|name| name.into_owned()),
            major: product.major.map(|major| major.into_owned()),
            minor: product.minor.map(|minor| minor.into_owned()),
            patch: product.patch.map(|patch| patch.into_owned()),
        }
    }
}

#[derive(Debug)]
pub struct Device {
    pub name: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
}

impl From<user_agent_parser::Device<'_>> for Device {
    fn from(device: user_agent_parser::Device) -> Self {
        Device {
            name: device.name.map(|name| name.into_owned()),
            brand: device.brand.map(|brand| brand.into_owned()),
            model: device.model.map(|model| model.into_owned()),
        }
    }
}

#[derive(Debug)]
pub struct UserAgent {
    pub product: Product,
    pub os: OS,
    pub device: Device,
}

impl FromRequest for UserAgent {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        use user_agent_parser::UserAgentParser;
        if let Some(ua) = req.headers().get("User-Agent") {
            match req.app_data::<actix_web::web::Data<UserAgentParser>>() {
                Some(ua_parser) => {
                    let ua = ua.to_str().unwrap();

                    let product = ua_parser.parse_product(ua);
                    let os = ua_parser.parse_os(ua);
                    let device = ua_parser.parse_device(ua);

                    ok(UserAgent {
                        product: Product::from(product),
                        os: OS::from(os),
                        device: Device::from(device),
                    })
                }
                _ => err(ErrorBadRequest("no luck")),
            }
            // use user_agent_parser::UserAgentParser;
            // let ua_parser = UserAgentParser::from_path("regexes.yaml").unwrap();
            // let ua = ua.to_str().unwrap();

            // let product = ua_parser.parse_product(ua);
            // let os = ua_parser.parse_os(ua);
            // let device = ua_parser.parse_device(ua);

            // ok(UserAgent {
            //     product: Product::from(product),
            //     os: OS::from(os),
            //     device: Device::from(device),
            // })
        } else {
            err(ErrorBadRequest("no luck"))
        }
    }
}
