use reqwest::Client;
use crate::model::models::EmailRequest;

pub struct EmailRepository {
    client: Client,
    api_key: String,
}

impl EmailRepository {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn send_email(&self, email: EmailRequest) -> Result<(), reqwest::Error> {
        let url = "https://api.brevo.com/v3/smtp/email";

        let response = self.client.post(url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&email)
            .send()
            .await?
            .error_for_status();

        match response {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
