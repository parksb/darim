use diesel::MysqlConnection;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::models::auth::password_token::{PasswordToken, PasswordTokenRepository};
use crate::models::error::Result;
use crate::models::user::UserRepository;
use crate::utils::email_util;
use crate::utils::env_util::CLIENT_ADDRESS;

pub struct PasswordTokenService<'a> {
    password_token_repository: PasswordTokenRepository,
    user_repository: UserRepository<'a>,
}

impl<'a> PasswordTokenService<'a> {
    pub fn new(conn: &'a MysqlConnection) -> Self {
        Self {
            password_token_repository: PasswordTokenRepository::new(),
            user_repository: UserRepository::new(conn),
        }
    }

    /// Sets token for temporary password deposition in password finding process.
    pub fn set(&mut self, email: &str) -> Result<bool> {
        let user = self.user_repository.find_by_email(email)?;

        let token = PasswordToken {
            id: thread_rng().sample_iter(&Alphanumeric).take(32).collect(),
            password: thread_rng().sample_iter(&Alphanumeric).take(512).collect(),
        };

        let serialized_token = serde_json::to_string(&token)?;
        let result = self
            .password_token_repository
            .save(user.id, &serialized_token)?;

        let _ = email_util::send_email(
            &format!("{} <{}>", user.name, email),
            &String::from("Please reset your password 🔒"),
            &self.email_content(&token),
        );

        Ok(result)
    }

    fn email_content(&self, token: &PasswordToken) -> String {
        format!(
            "Hello :)<br/><br/>\
            Please copy the temporary password:<br/><br/>\
            <div style=\"background-color: #f0f0f0; padding: 10px; font-weight: bold\">{}</div><br/><br/>\
            and visit the link to reset your password:<br/><br/>\
            <a href=\"{}/password_reset/{}\">{}/password_reset/{}</a>",
            token.password, *CLIENT_ADDRESS, token.id, *CLIENT_ADDRESS, token.id,
        )
    }
}
