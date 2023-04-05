use diesel::MysqlConnection;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::models::auth::password_token::{PasswordToken, PasswordTokenRepository};
use crate::models::connection::RedisConnection;
use crate::models::error::{Error, Result};
use crate::models::user::UserRepository;
use crate::utils::env_util::CLIENT_ADDRESS;
use crate::utils::{argon2_password_util, email_util};

pub struct PasswordService<'a> {
    password_token_repository: PasswordTokenRepository<'a>,
    user_repository: UserRepository<'a>,
}

impl<'a> PasswordService<'a> {
    pub fn new(rdb_conn: &'a MysqlConnection, redis_conn: &'a mut RedisConnection) -> Self {
        Self {
            password_token_repository: PasswordTokenRepository::new(redis_conn),
            user_repository: UserRepository::new(rdb_conn),
        }
    }

    /// Sets token for temporary password deposition in password finding process.
    pub async fn set(&mut self, email: &str) -> Result<bool> {
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
        ).await;

        Ok(result)
    }

    // Reset the password.
    pub fn reset(
        &mut self,
        email: &str,
        token_id: &str,
        temporary_password: &str,
        new_password: &str,
    ) -> Result<bool> {
        let user = self.user_repository.find_by_email(email)?;

        let token: PasswordToken = {
            let serialized_token = self.password_token_repository.find(user.id)?;
            serde_json::from_str(&serialized_token)?
        };

        if token.id == token_id && token.password == temporary_password {
            let password_salt: String = argon2_password_util::generate_password_salt();
            let hashed_password =
                argon2_password_util::hash_password(new_password, &password_salt)?;
            self.user_repository
                .update(user.id, &None, &Some(hashed_password), &None)?;
            self.password_token_repository.delete(user.id)
        } else {
            Err(Error::UserNotFound(email.to_string()))
        }
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
