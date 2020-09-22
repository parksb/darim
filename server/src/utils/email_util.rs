use lettre::header::ContentType;
use lettre::message::{Message, SinglePart};
use lettre::{SendmailTransport, Transport};
use std::env;

use crate::models::error::ServiceError;

pub fn send_email(to: &str, subject: &str, body: &str) -> Result<bool, ServiceError> {
    let email_address = env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS not found");
    let parsed_email_address = email_address.parse().unwrap();
    let email = Message::builder()
        .from(parsed_email_address)
        .to(to.parse().unwrap())
        .subject(subject)
        .singlepart(
            SinglePart::builder()
                .header(ContentType("text/html; charset=utf8".parse().unwrap()))
                .body(body),
        )
        .unwrap();

    let sender = SendmailTransport::new();
    match sender.send(&email) {
        Ok(_) => Ok(true),
        Err(_) => Err(ServiceError::EmailFailure(to.to_string())),
    }
}
