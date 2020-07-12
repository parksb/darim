use actix_session::Session;
use actix_web::{delete, patch, post, web, Responder};

use crate::models::{error::*, user::*};
use crate::services::user::UserService;
use crate::utils::{http_util, session_util};

/// User route
pub struct UserRoute {}

impl UserRoute {
    /// Creates a new user
    ///
    /// # Request
    ///
    /// ```text
    /// POST /users
    /// ```
    ///
    /// ## Parameters
    ///
    /// * user_public_key - A user's public key
    /// * token_key - A key for token search
    /// * token_pin - A pin for verifying
    ///
    /// ```json
    /// {
    ///     "user_public_key": "d63ee429"
    ///     "token_key": "71I3Qz9u",
    ///     "token_pin": "P9d82Jc5"
    /// }
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": true,
    ///     "error": null
    /// }
    /// ```
    pub fn create_user(args: web::Json<CreateArgs>) -> impl Responder {
        let response = UserService::create(args.into_inner());
        http_util::get_response::<bool>(response)
    }

    /// Deletes a user
    ///
    /// # Request
    ///
    /// ```text
    /// DELETE /users/:id
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": true,
    ///     "error": null
    /// }
    /// ```
    pub fn delete_user(session: Session, id: web::Path<u64>) -> impl Responder {
        let response = if let Some(user_session) = session_util::get_session(&session) {
            let id_in_path = id.into_inner();
            if id_in_path != user_session.user_id {
                Err(ServiceError::Unauthorized)
            } else {
                UserService::delete(id_in_path)
            }
        } else {
            Err(ServiceError::Unauthorized)
        };

        http_util::get_response::<bool>(response)
    }

    /// Updates a user
    ///
    /// # Request
    ///
    /// ```text
    /// PATCH /users/:id
    /// ```
    ///
    /// ## Parameters
    ///
    /// * name - A name of the user.
    /// * password - A password of the user.
    /// * avatar_url - An avatar image url of the user.
    ///
    /// ```json
    /// {
    ///     "name": "park",
    ///     "password": "Ir5c7y8dS3",
    ///     "avatar_url": "avatar.jpg"
    /// }
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": true,
    ///     "error": null
    /// }
    /// ```
    pub fn update_user(
        session: Session,
        id: web::Path<u64>,
        user: web::Json<UpdateArgs>,
    ) -> impl Responder {
        let response = if let Some(user_session) = session_util::get_session(&session) {
            let id_in_path = id.into_inner();
            if id_in_path != user_session.user_id {
                Err(ServiceError::Unauthorized)
            } else {
                UserService::update(id_in_path, user.into_inner())
            }
        } else {
            Err(ServiceError::Unauthorized)
        };

        http_util::get_response::<bool>(response)
    }
}

#[post("/users")]
pub async fn create_user_route(args: web::Json<CreateArgs>) -> impl Responder {
    UserRoute::create_user(args)
}

#[delete("/users/{id}")]
pub async fn delete_user_route(session: Session, id: web::Path<u64>) -> impl Responder {
    UserRoute::delete_user(session, id)
}

#[patch("/users/{id}")]
pub async fn update_user_route(
    session: Session,
    id: web::Path<u64>,
    user: web::Json<UpdateArgs>,
) -> impl Responder {
    UserRoute::update_user(session, id, user)
}

/// Initializes the user routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user_route);
    cfg.service(delete_user_route);
    cfg.service(update_user_route);
}
