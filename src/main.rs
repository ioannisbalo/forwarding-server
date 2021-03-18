use actix_web::{App, HttpServer};
use server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = server::Configuration::new().port;

    println!("Starting server on port {}", port);

    HttpServer::new(move || {
        App::new()
        .service(server::scope())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
