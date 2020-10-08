use actix_web::{delete, patch, post, web, Responder};
use serde::{Deserialize, Serialize};

use crate::services::user::UserService;
use crate::utils::http_util;

/// Arguments for `POST /users` API.
#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub user_public_key: String,
    pub token_key: String,
    pub token_pin: String,
    pub recaptcha_token: String,
}

/// Arguments for `PATCH /users/:id` API.
#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub name: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
}

/// Arguments for `POST /users/password` API.
#[derive(Serialize, Deserialize)]
pub struct ResetPasswordArgs {
    pub email: String,
    pub token_id: String,
    pub temporary_password: String,
    pub new_password: String,
}

/// Creates a new user
#[post("/users")]
pub async fn create_user(args: web::Json<CreateArgs>) -> impl Responder {
    let CreateArgs {
        user_public_key,
        token_key,
        token_pin,
        recaptcha_token,
    } = args.into_inner();
    let result = UserService::new()
        .create(&user_public_key, &token_key, &token_pin, &recaptcha_token)
        .await;
    http_util::get_response::<bool>(result)
}

/// Deletes a user
#[delete("/users/{id}")]
pub async fn delete_user(id: web::Path<u64>) -> impl Responder {
    let result = UserService::new().delete(id.into_inner());
    http_util::get_response::<bool>(result)
}

/// Updates a user
#[patch("/users/{id}")]
pub async fn update_user(id: web::Path<u64>, args: web::Json<UpdateArgs>) -> impl Responder {
    let UpdateArgs {
        name,
        password,
        avatar_url,
    } = args.into_inner();
    let result = UserService::new().update(id.into_inner(), &name, &password, &avatar_url);
    http_util::get_response::<bool>(result)
}

/// Resets the password.
#[post("/users/password")]
pub async fn reset_password(args: web::Json<ResetPasswordArgs>) -> impl Responder {
    let ResetPasswordArgs {
        email,
        token_id,
        temporary_password,
        new_password,
    } = args.into_inner();
    let result =
        UserService::new().reset_password(&email, &token_id, &temporary_password, &new_password);
    http_util::get_response::<bool>(result)
}

/// Initializes the user routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(delete_user);
    cfg.service(update_user);
    cfg.service(reset_password);
}
