use serde::Serialize;

#[derive(Serialize)]
pub struct EmailRequest {
    pub sender: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub subject: String,
    pub htmlContent: String,
}

#[derive(Serialize)]
pub struct EmailAddress {
    pub email: String,
}
