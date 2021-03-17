use actix_web::{get, web, HttpResponse, Scope, Responder};

#[get("/readiness")]
async fn ready() -> impl Responder {
    HttpResponse::Ok().body("Ready")
}

pub fn scope() -> Scope {
    web::scope("/healthchecks").service(ready)
}
