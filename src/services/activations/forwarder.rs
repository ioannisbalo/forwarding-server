pub mod auth;

use actix_web::client::{Client, SendRequestError};
use super::entities::Activation;

pub enum ForwardResponse {
    Ok(String),
    Err(SendRequestError)
}

pub async fn forward_with_retries(url: &str, activation: Activation, mut retries: u8) -> ForwardResponse {
    let mut response = forward(url, &activation).await;

    while let ForwardResponse::Err(_) = response {
        if retries <= 0 { break; }
        response = forward(url, &activation).await;
        retries -= 1;
    }

    response
}

async fn forward(url: &str, activation: &Activation) -> ForwardResponse {
    let client: Client = Client::default();
    let mut request = client.post(url).header("User-Agent", "actix-web/3.0");
    request = auth::add_auth(request);
    println!("{:?}", request.headers());

    let http_response = request
            .send_json(&activation)
            .await;
    
    match http_response {
        Ok(res) => ForwardResponse::Ok(res.status().to_string()),
        Err(err) => ForwardResponse::Err(err)
    }
}