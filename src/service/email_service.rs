use crate::model::models::{EmailAddress, EmailRequest};
use crate::repository::email_repository::EmailRepository;


pub struct EmailService {
    repository: EmailRepository,
    sender_email: String,
}

impl EmailService {
    pub fn new(repository: EmailRepository, sender_email: String) -> Self {
        Self { repository, sender_email }
    }

    pub async fn send_email(&self, recipient: &str, subject: &str, content: &str) -> Result<(), reqwest::Error> {
        let email_request = EmailRequest {
            sender: EmailAddress { email: self.sender_email.clone() },
            to: vec![EmailAddress { email: recipient.to_string() }],
            subject: subject.to_string(),
            htmlContent: content.to_string(),
        };

        self.repository.send_email(email_request).await
    }
}
