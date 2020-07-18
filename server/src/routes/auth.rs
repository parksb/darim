use actix_session::Session;
use actix_web::{get, post, web, Responder};

use crate::models::{auth::*, error::*};
use crate::services::auth::AuthService;
use crate::utils::{http_util, session_util};

/// Auth route.
pub struct AuthRoute {}

impl AuthRoute {
    /// Responds auth information as user session.
    ///
    /// # Request
    ///
    /// ```text
    /// GET /auth
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": {
    ///         "user_id": 0,
    ///         "user_email": "park@email.com"
    ///         "user_name": "park",
    ///         "user_avatar_url": "avatar.jpg"
    ///     },
    ///     "error": null
    /// }
    /// ```
    pub fn get_auth(session: Session) -> impl Responder {
        let user_session = session_util::get_session(&session);

        if let Some(response) = user_session {
            http_util::get_response::<UserSession>(Ok(response))
        } else {
            http_util::get_response::<UserSession>(Err(ServiceError::Unauthorized))
        }
    }

    /// Refresh auth information as user session.
    ///
    /// # Request
    ///
    /// ```text
    /// POST /auth
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": {
    ///         "user_id": 0,
    //         "user_email": "park@email.com"
    ///        "user_name": "park",
    ///        "user_avatar_url": "avatar.jpg"
    ///     },
    ///     "error": null
    /// }
    /// ```
    pub fn refresh_auth(session: Session) -> impl Responder {
        let user_session = AuthService::refresh_user_session(session);
        http_util::get_response::<UserSession>(user_session)
    }

    /// Sets token for creating user.
    ///
    /// # Request
    ///
    /// ```text
    /// POST /auth/token/sign_up
    /// ```
    ///
    /// ## Parameters
    ///
    /// * name - A name of the user.
    /// * email - A unique email of the user.
    /// * password - A password of the user.
    /// * avatar_url - An avatar image url of the user.
    ///
    /// ```json
    /// {
    ///     "name": "park",
    ///     "email": "park@email.com",
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
    pub fn set_sign_up_token(args: web::Json<SetSignUpTokenArgs>) -> impl Responder {
        let response = AuthService::set_sign_up_token(args.into_inner());
        http_util::get_response::<bool>(response)
    }

    /// Sets token for resetting password.
    ///
    /// # Request
    ///
    /// ```text
    /// POST /auth/token/password
    /// ```
    ///
    /// ## Parameters
    ///
    /// * email - A unique email of the user.
    ///
    /// ```json
    /// {
    ///     "email": "park@email.com",
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
    pub fn set_password_token(args: web::Json<SetPasswordTokenArgs>) -> impl Responder {
        let response = AuthService::set_password_token(args.into_inner());
        http_util::get_response::<bool>(response)
    }

    /// Signs in to set user session.
    ///
    /// # Request
    ///
    /// ```text
    /// POST /auth/login
    /// ```
    ///
    /// ## Parameters
    ///
    /// * email - A unique email of the user.
    /// * password - A password of the user.
    ///
    /// ```json
    /// {
    ///     "email": "park@email.com",
    ///     "password": "Ir5c7y8dS3",
    /// }
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": {
    ///         "user_id": 0,
    ///         "user_email": "park@email.com"
    ///         "user_name": "park",
    ///     },
    ///     "error": null
    /// }
    /// ```
    pub fn login(mut session: Session, args: web::Json<LoginArgs>) -> impl Responder {
        let user_session = AuthService::login(args.into_inner());

        match user_session {
            Ok(response) => {
                let is_succeed = session_util::set_session(
                    &mut session,
                    response.user_id,
                    &response.user_email,
                    &response.user_name,
                    &response.user_public_key,
                    &response.user_avatar_url,
                );

                if is_succeed {
                    http_util::get_response::<UserSession>(Ok(response))
                } else {
                    http_util::get_response::<UserSession>(Err(ServiceError::InternalServerError))
                }
            }
            Err(ServiceError::NotFound(key)) => {
                http_util::get_response::<UserSession>(Err(ServiceError::NotFound(key)))
            }
            _ => http_util::get_response::<UserSession>(Err(ServiceError::InternalServerError)),
        }
    }

    /// Signs out to unset user session.
    ///
    /// # Request
    ///
    /// ```text
    /// POST /auth/logout
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
    pub fn logout(mut session: Session) -> impl Responder {
        let is_logged_in = session_util::get_session(&session);
        let result = if is_logged_in.is_some() {
            session_util::unset_session(&mut session);
            true
        } else {
            false
        };

        http_util::get_response::<bool>(Ok(result))
    }
}

#[get("/auth")]
pub async fn get_auth_route(session: Session) -> impl Responder {
    AuthRoute::get_auth(session)
}

#[post("/auth")]
pub async fn refresh_session_route(session: Session) -> impl Responder {
    AuthRoute::refresh_auth(session)
}

#[post("/auth/token/sign_up")]
pub async fn set_sign_up_token_route(args: web::Json<SetSignUpTokenArgs>) -> impl Responder {
    AuthRoute::set_sign_up_token(args)
}

#[post("/auth/token/password")]
pub async fn set_password_token_route(args: web::Json<SetPasswordTokenArgs>) -> impl Responder {
    AuthRoute::set_password_token(args)
}

#[post("/auth/login")]
pub async fn login_route(session: Session, args: web::Json<LoginArgs>) -> impl Responder {
    AuthRoute::login(session, args)
}

#[post("/auth/logout")]
pub async fn logout_route(session: Session) -> impl Responder {
    AuthRoute::logout(session)
}

/// Initializes the auth routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(set_sign_up_token_route);
    cfg.service(set_password_token_route);
    cfg.service(refresh_session_route);
    cfg.service(get_auth_route);
    cfg.service(login_route);
    cfg.service(logout_route);
}
