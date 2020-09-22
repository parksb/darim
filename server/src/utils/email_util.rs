use lettre::{Message, SmtpTransport, Transport};
use std::env;

use crate::models::error::ServiceError;

pub fn send_email(to: &str, subject: &str, body: &str) -> Result<bool, ServiceError> {
    let email_address = env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS not found");
    let smtp_relay_address = env::var("SMTP_RELAY_ADDRESS").expect("SMTP_RELAY_ADDRESS not found");

    let parsed_email_address = email_address.parse().unwrap();

    let email = Message::builder()
        .from(parsed_email_address)
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let mailer = SmtpTransport::relay(&smtp_relay_address).unwrap().build();

    match mailer.send(&email) {
        Ok(_) => Ok(true),
        Err(_) => Err(ServiceError::EmailFailure(to.to_string())),
    }
}
