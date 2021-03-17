pub mod controllers;
pub mod services;
pub mod config;
pub use config::Configuration;

use actix_web::{web, Scope};
use controllers::{healthchecks, activations};

pub fn scope() -> Scope {
    web::scope("/api")
        .service(healthchecks::scope())
        .service(activations::scope())
}
