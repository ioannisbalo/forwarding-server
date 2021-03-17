use actix_web::client::ClientRequest;
use crate::Configuration;

pub enum AuthStrategy {
    Okta,
    Basic,
    None
}

impl From<&str> for AuthStrategy {
    fn from(strategy: &str) -> Self {
        match strategy {
            "Okta" => Self::Okta,
            "Basic" => Self::Basic,
            _ => Self::None
        }
    }
}

pub fn add_auth(request: ClientRequest) -> ClientRequest {
    let config = Configuration::new();
    let strategy = AuthStrategy::from(&config.authentication_strategy[..]);

    match strategy {
        AuthStrategy::Okta => okta(request, config),
        AuthStrategy::Basic => basic(request, config),
        AuthStrategy::None => none(request, config)
    }
}

fn okta(request: ClientRequest, _config: Configuration) -> ClientRequest {
    request
}

fn basic(request: ClientRequest, config: Configuration) -> ClientRequest {
    request.basic_auth(config.username, Some(&config.password))
}

fn none(request: ClientRequest, _config: Configuration) -> ClientRequest {
    request
}