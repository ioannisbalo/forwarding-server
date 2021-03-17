mod entities;
pub mod forwarder;

use entities::Activation;
use forwarder::*;
use crate::controllers::activations::dto;

pub async fn process(activation_dto: dto::Activation, url: &str) -> ForwardResponse {
    let activation = Activation::from(activation_dto);
    
    forward_with_retries(url, activation, 3).await
}
