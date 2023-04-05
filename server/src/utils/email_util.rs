use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};

use crate::models::error::{Error, Result};
use crate::utils::env_util::{Profile, EMAIL_ADDRESS, PROFILE};

use super::env_util::{MAILJET_API_KEY, MAILJET_API_SECRET_KEY};

pub async fn send_email(to: &str, subject: &str, body: &str) -> Result<bool> {
    match *PROFILE {
        Profile::PRODUCTION => {
            let client = Client::new(
                SendAPIVersion::V3,
                &MAILJET_API_KEY,
                &MAILJET_API_SECRET_KEY,
            );

            let mut message =
                Message::new(&EMAIL_ADDRESS, "Darim", Some(subject.to_string()), None);
            message.push_recipient(Recipient::new(to));
            message.html_part = Some(body.to_string());

            match client.send(message).await {
                Ok(_) => Ok(true),
                Err(_) => Err(Error::MailError),
            }
        }
        Profile::DEV => {
            println!("to: {to}\nsubject: {subject}\nbody: {body}");
            Ok(true)
        }
    }
}
