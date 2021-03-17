use std::env;

pub struct Configuration {
  pub forwarding_url: String,
  pub port: String,
  pub authentication_strategy: String,
  pub username: String,
  pub password: String
}

impl Configuration {
  pub fn new() -> Self {
    Configuration {
      forwarding_url: if let Ok(var) = env::var("FORWARDING_URL") {var} else {String::from("http://localhost:4000/api/activations/hello")},
      port: if let Ok(var) = env::var("PORT") {var} else {String::from("4000")},
      authentication_strategy: if let Ok(var) = env::var("AUTHENTICATION_STRATEGY") {var} else {String::from("Basic")},
      username: if let Ok(var) = env::var("USERNAME") {var} else {String::from("user")},
      password: if let Ok(var) = env::var("PASSWORD") {var} else {String::from("pass")},
    }
  }
}
