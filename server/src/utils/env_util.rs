use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref HOST: String = env::var("HOST").expect("HOST not found");
    pub static ref PORT: String = env::var("PORT").expect("PORT not found");
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    pub static ref REDIS_URL: String = env::var("REDIS_URL").expect("REDIS_URL not found");
    pub static ref EMAIL_ADDRESS: String =
        env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS not found");
    pub static ref CLIENT_ADDRESS: String =
        env::var("CLIENT_ADDRESS").expect("CLIENT_ADDRESS not found");
    pub static ref RECAPTCHA_SECRET_KEY: String =
        env::var("RECAPTCHA_SECRET_KEY").expect("RECAPTCHA_SECRET_KEY not found");
    pub static ref JWT_REFRESH_SECRET: String =
        env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET not found");
    pub static ref PROFILE: Profile = env::var("PROFILE")
        .map(|profile| if profile == "production" {
            Profile::PRODUCTION
        } else {
            Profile::DEV
        })
        .unwrap_or(Profile::DEV);
}

#[derive(PartialEq)]
pub enum Profile {
    DEV,
    PRODUCTION,
}
