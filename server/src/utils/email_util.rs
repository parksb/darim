use lettre::message::header::ContentType;
use lettre::message::{Message, SinglePart};
use lettre::transport::sendmail::SendmailTransport;
use lettre::Transport;

use crate::models::error::Result;
use crate::utils::env_util::{Profile, EMAIL_ADDRESS, PROFILE};

pub fn send_email(to: &str, subject: &str, body: &str) -> Result<bool> {
    match *PROFILE {
        Profile::PRODUCTION => {
            let parsed_email_address = (*EMAIL_ADDRESS).parse()?;
            let email = Message::builder()
                .from(parsed_email_address)
                .to(to.parse().unwrap())
                .subject(subject)
                .singlepart(
                    SinglePart::builder()
                        .content_type(ContentType::TEXT_HTML)
                        .body(body.to_string()),
                )?;

            let sender = SendmailTransport::new();
            let _ = sender.send(&email)?;

            Ok(true)
        }
        Profile::DEV => {
            println!("to: {}\nsubject: {}\nbody: {}", to, subject, body);
            Ok(true)
        }
    }
}
