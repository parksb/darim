use chrono::{Duration, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::models::error::{Error, Result};
use crate::utils::env_util::JWT_REFRESH_SECRET;

/// JWT Claims
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub user_id: u64,
}

impl Claims {
    pub fn new(user_id: u64) -> Self {
        let current_date_time = Utc::now();
        Self {
            user_id,
            exp: (current_date_time + Duration::days(30)).timestamp(),
            iat: current_date_time.timestamp(),
        }
    }

    pub fn from_token(token: &str) -> Result<Claims> {
        let validation = Validation::default();
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_REFRESH_SECRET.as_ref()),
            &validation,
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(error) => match error.kind() {
                ErrorKind::InvalidToken => Err(Error::InvalidToken),
                ErrorKind::ExpiredSignature => Err(Error::ExpiredToken),
                _ => Err(Error::InternalServerError),
            },
        }
    }
}
