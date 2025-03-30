use crate::service::email_service::EmailService;

pub struct EmailController {
    service: EmailService,
}

impl EmailController {
    pub fn new(service: EmailService) -> Self {
        Self { service }
    }

    pub async fn send_test_email(&self) {
        let recipient = "ejioforkelvin@gmail.com";
        let subject = "Test Email from Rust";
        let content = "<p>Hello! This is a test email sent using Brevo in Rust.</p>";

        match self.service.send_email(recipient, subject, content).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => eprintln!("Failed to send email: {}", e),
        }
    }
}
