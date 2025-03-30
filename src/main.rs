mod config;
mod model;
mod repository;
mod service;
mod controller;

use config::Config;
use repository::email_repository::EmailRepository;
use service::email_service::EmailService;
use controller::email_controller::EmailController;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::init();
    let email_repository = EmailRepository::new(config.brevo_api_key);
    let email_service = EmailService::new(email_repository, config.sender_email);
    let email_controller = EmailController::new(email_service);

    email_controller.send_test_email().await;
}
