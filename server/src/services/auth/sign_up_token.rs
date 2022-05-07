use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::models::auth::sign_up_token::{SignUpToken, SignUpTokenRepository};
use crate::models::connection::RedisConnection;
use crate::models::error::{Error, Result};
use crate::utils::{argon2_password_util, email_util};

pub struct SignUpTokenService<'a> {
    sign_up_token_repository: SignUpTokenRepository<'a>,
}

impl<'a> SignUpTokenService<'a> {
    pub fn new(conn: &'a mut RedisConnection) -> Self {
        Self {
            sign_up_token_repository: SignUpTokenRepository::new(conn),
        }
    }

    /// Sets token for sign up process.
    ///
    /// 1. Generates a random string called pin.
    /// 2. Creates a new token containing the pin and information of the user from arguments.
    /// 3. Serializes the token and inserts it to redis.
    pub fn set(
        &mut self,
        name: &str,
        email: &str,
        password: &str,
        avatar_url: &Option<String>,
    ) -> Result<String> {
        if name.trim().is_empty() || email.trim().is_empty() || password.trim().is_empty() {
            return Err(Error::InvalidArgument);
        }

        let pin: String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
        let password_salt: String = argon2_password_util::generate_password_salt();
        let hashed_password = argon2_password_util::hash_password(password, &password_salt)?;

        let token = SignUpToken {
            pin,
            name: name.to_string(),
            email: email.to_string(),
            password: hashed_password,
            avatar_url: avatar_url.clone(),
        };

        let serialized_token = serde_json::to_string(&token)?;
        let result = self.sign_up_token_repository.save(&serialized_token)?;

        let _ = email_util::send_email(
            &format!("{} <{}>", &token.name, &token.email),
            &String::from("Welcome to Darim 🎉"),
            &self.email_content(&token),
        )?;

        Ok(result)
    }

    fn email_content(&self, token: &SignUpToken) -> String {
        format!(
            "<h1>🏕 Welcome to Darim</h1>\
            <h2>Hello {} :)</h2>\
            You've joined Darim.<br/><br/>\
            Please copy the key below to finish the sign up process:<br/><br/>\
            <div style=\"background-color: #f0f0f0; padding: 10px; font-size: 20px; font-weight: bold\">{}</div>",
            token.name, token.pin,
        )
    }
}
