use crate::models::auth::UserSession;
use actix_session::Session;

/// Sets user session.
///
/// # Arguments
///
/// * `session` - An session object
/// * `user_id` - A record id of the user account
/// * `user_email` -  An email of the user account
/// * `user_name` - A name of the user account
/// * `user_avatar_url` - A avatar image url of the user account
pub fn set_session(
    session: Session,
    user_id: u64,
    user_email: &str,
    user_name: &str,
    user_avatar_url: &Option<String>,
) -> bool {
    let is_set_user_id = session.set("user_id", user_id);
    let is_set_user_email = session.set("user_email", user_email);
    let is_set_user_name = session.set("user_name", user_name);

    let is_set_user_avatar_url = if let Some(user_avatar_url) = user_avatar_url {
        session.set("user_avatar_url", user_avatar_url)
    } else {
        Ok(())
    };

    !(is_set_user_id.is_err()
        || is_set_user_email.is_err()
        || is_set_user_name.is_err()
        || is_set_user_avatar_url.is_err())
}

/// Clears session.
///
/// # Arguments
///
/// * `session` - An session object
pub fn unset_session(session: Session) {
    session.clear();
}

/// Returns user session.
///
/// # Arguments
///
/// * `session` - An session object
pub fn get_session(session: &Session) -> Option<UserSession> {
    let user_id = if let Ok(id) = session.get::<u64>("user_id") {
        if let Some(id) = id {
            id
        } else {
            return None;
        }
    } else {
        return None;
    };

    let user_email = if let Ok(email) = session.get::<String>("user_email") {
        if let Some(email) = email {
            email
        } else {
            return None;
        }
    } else {
        return None;
    };

    let user_name = if let Ok(name) = session.get::<String>("user_name") {
        if let Some(name) = name {
            name
        } else {
            return None;
        }
    } else {
        return None;
    };

    let user_avatar_url = if let Ok(avatar_url) = session.get::<String>("user_avatar_url") {
        avatar_url
    } else {
        return None;
    };

    Some(UserSession {
        user_id,
        user_email,
        user_name,
        user_avatar_url,
    })
}
