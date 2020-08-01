use lettre::error::Error;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

pub fn send_email(to: &str, subject: &str, body: &str) -> Result<bool, Error> {
    let email = EmailBuilder::new()
        .from("root@harooo.com")
        .to((to, ""))
        .subject(subject)
        .body(body)
        .build();

    if let Ok(email) = email {
        let smtp_client = SmtpClient::new_unencrypted_localhost();
        if let Ok(smtp_client) = smtp_client {
            let result = smtp_client.transport().send(email.into());
            if result.is_ok() {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}
