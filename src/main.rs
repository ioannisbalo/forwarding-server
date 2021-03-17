use actix_web::{App, HttpServer};
use server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = server::Configuration::new().port;

    HttpServer::new(move || {
        App::new()
        .service(server::scope())
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
