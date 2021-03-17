pub mod dto;

use actix_web::{post, web, HttpResponse, Scope, Responder};
use dto::Activation;
use crate::services::activations;
use crate::services::activations::forwarder::ForwardResponse;
use crate::Configuration;

#[post("/")]
async fn process(activation_dto: web::Json<Activation>) -> impl Responder {
    let url = Configuration::new().forwarding_url;

    match activations::process(activation_dto.into_inner(), &url).await {
        ForwardResponse::Ok(res) => HttpResponse::Ok().body(res),
        ForwardResponse::Err(err) => {
            println!("{}", err);
            HttpResponse::Ok().body("error")
        }
    }
}

#[post("/hello")]
async fn test(activation_dto: web::Json<Activation>) -> impl Responder {
    println!("{}", activation_dto.id);
    
    HttpResponse::Ok().body("bro")
}

pub fn scope() -> Scope {
    web::scope("/activations").service(process).service(test)
}
