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
            name: os.name.and_then(|name| Some(name.into_owned())),
            major: os.major.and_then(|major| Some(major.into_owned())),
            minor: os.minor.and_then(|minor| Some(minor.into_owned())),
            patch: os.patch.and_then(|patch| Some(patch.into_owned())),
            patch_minor: os
                .patch_minor
                .and_then(|patch_minor| Some(patch_minor.into_owned())),
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
            name: product.name.and_then(|name| Some(name.into_owned())),
            major: product.major.and_then(|major| Some(major.into_owned())),
            minor: product.minor.and_then(|minor| Some(minor.into_owned())),
            patch: product.patch.and_then(|patch| Some(patch.into_owned())),
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
            name: device.name.and_then(|name| Some(name.into_owned())),
            brand: device.brand.and_then(|brand| Some(brand.into_owned())),
            model: device.model.and_then(|model| Some(model.into_owned())),
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
        if let Some(ua) = req.headers().get("User-Agent") {
            use user_agent_parser::UserAgentParser;
            let ua_parser = UserAgentParser::from_path("regexes.yaml").unwrap();
            let ua = ua.to_str().unwrap();

            let product = ua_parser.parse_product(ua);
            let os = ua_parser.parse_os(ua);
            let device = ua_parser.parse_device(ua);

            ok(UserAgent {
                product: Product::from(product),
                os: OS::from(os),
                device: Device::from(device),
            })
        } else {
            err(ErrorBadRequest("no luck"))
        }
    }
}
