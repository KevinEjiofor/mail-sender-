use dotenv::dotenv;
use std::env;

pub struct Config {
    pub brevo_api_key: String,
    pub sender_email: String,
}

impl Config {
    pub fn init() -> Self {
        dotenv().ok();
        Self {
            brevo_api_key: env::var("BREVO_API_KEY").expect("BREVO_API_KEY not set"),
            sender_email: env::var("SENDER_EMAIL").expect("SENDER_EMAIL not set"),
        }
    }
}
